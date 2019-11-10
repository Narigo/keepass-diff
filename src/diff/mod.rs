use std::collections::{HashMap, HashSet};
use termcolor::Color;

use string_stack::StringStack;

pub mod entry;
pub mod field;
pub mod group;

/// The possible outcomes of diffing two objects against another
#[derive(Debug)]
pub enum DiffResult<'a, T> {
    /// The objects are identical, including any children
    Identical { left: &'a T, right: &'a T },
    /// The objects have changed value
    Changed { left: &'a T, right: &'a T },
    /// There is a difference in a child object
    InnerDifferences {
        left: &'a T,
        right: &'a T,
        inner_differences: Vec<Box<dyn DiffResultFormat + 'a>>,
    },
    /// Only the left object exists
    OnlyLeft { left: &'a T },
    /// Only the right object exists
    OnlyRight { right: &'a T },
}

/// Denotes that an object can be diffed
pub trait Diff
where
    Self: Sized,
{
    fn diff<'a>(&'a self, other: &'a Self) -> DiffResult<'a, Self>;
}

/// Denotes that an object can be formatted as a DiffResult
pub trait DiffResultFormat: std::fmt::Debug {
    fn diff_result_format(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        path: StringStack,
        use_color: bool,
    ) -> std::fmt::Result;
}

/// Helper wrapper to impl Display for a DiffResult with user-specified settings
pub struct DiffDisplay<T: DiffResultFormat> {
    pub inner: T,
    pub path: StringStack,
    pub use_color: bool,
}

impl<T: DiffResultFormat> std::fmt::Display for DiffDisplay<T> {
    fn fmt(&self, mut f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner
            .diff_result_format(&mut f, self.path.copy(), self.use_color)
    }
}

/// Format functionality for deep recursion
impl<'a, E> DiffResultFormat for DiffResult<'a, E>
where
    E: std::fmt::Display + std::fmt::Debug,
{
    fn diff_result_format(
        &self,
        mut f: &mut std::fmt::Formatter<'_>,
        path: StringStack,
        use_color: bool,
    ) -> std::fmt::Result {
        let _ = match self {
            DiffResult::Identical { .. } => Ok(()),
            DiffResult::Changed { left, right } => {
                if use_color {
                    crate::set_fg(Some(Color::Red));
                }
                write!(f, "- {}\n", path.push(format!("{}", left)).mk_string("[", " > ", "]"))?;
                if use_color {
                    crate::set_fg(Some(Color::Green));
                }
                write!(f, "+ {}\n", path.push(format!("{}", right)).mk_string("[", " > ", "]"))
            }
            DiffResult::InnerDifferences {
                left,
                inner_differences,
                ..
            } => {
                if use_color {
                    crate::set_fg(Some(Color::Yellow));
                }
                // write!(f, "~ {}{}\n", indent, left)?;
                for id in inner_differences {
                    id.diff_result_format(
                        &mut f,
                        path.push(format!("{}", left)),
                        use_color,
                    )?;
                }
                Ok(())
            }
            DiffResult::OnlyLeft { left } => {
                if use_color {
                    crate::set_fg(Some(Color::Red));
                }
                write!(f, "- {}\n", path.push(format!("{}", left)).mk_string("[", " > ", "]"))
            }
            DiffResult::OnlyRight { right } => {
                if use_color {
                    crate::set_fg(Some(Color::Green));
                }
                write!(f, "+ {}\n", path.push(format!("{}", right)).mk_string("[", " > ", "]"))
            }
        };

        if use_color {
            crate::set_fg(None);
        }

        Ok(())
    }
}

/// Compare to HashMaps of the same value type with each other, returning a bool indicating whether
/// there are any differences and a Vec<DiffResult<A>> listing all differences
pub fn diff_hashmap<'a, A>(
    a: &'a HashMap<String, A>,
    b: &'a HashMap<String, A>,
) -> (bool, Vec<DiffResult<'a, A>>)
where
    A: Diff,
{
    let mut keys = HashSet::new();
    keys.extend(a.keys());
    keys.extend(b.keys());

    let mut keys: Vec<_> = keys.iter().collect();
    keys.sort();

    let mut acc: Vec<DiffResult<A>> = Vec::new();

    let mut has_differences = false;

    for key in keys {
        let el_a: Option<&A> = a.get(*key);
        let el_b: Option<&A> = b.get(*key);

        match (el_a, el_b) {
            // both a and b have the key
            (Some(v_a), Some(v_b)) => {
                let dr: DiffResult<A> = v_a.diff(v_b);

                if let DiffResult::Identical { .. } = dr {
                } else {
                    has_differences = true;
                }

                acc.push(dr);
            }

            // only a has the key
            (Some(v_a), None) => {
                has_differences = true;
                acc.push(DiffResult::OnlyLeft { left: v_a })
            }

            // only b has the key
            (None, Some(v_b)) => {
                has_differences = true;
                acc.push(DiffResult::OnlyRight { right: v_b })
            }

            // none have the key (this shouldn't happen)
            (None, None) => {}
        }
    }

    (has_differences, acc)
}
