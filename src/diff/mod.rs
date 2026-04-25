use std::collections::{HashMap, HashSet};
use termcolor::Color;

use stack::Stack;

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
        path: &Stack<&String>,
        use_color: bool,
        use_verbose: bool,
        mask_passwords: bool,
    ) -> std::fmt::Result;
}

/// Helper wrapper to impl Display for a DiffResult with user-specified settings
pub struct DiffDisplay<'a, T: DiffResultFormat> {
    pub inner: T,
    pub path: Stack<&'a String>,
    pub use_color: bool,
    pub use_verbose: bool,
    pub mask_passwords: bool,
}

impl<'a, T: DiffResultFormat> std::fmt::Display for DiffDisplay<'a, T> {
    fn fmt(&self, mut f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self.inner.diff_result_format(
            &mut f,
            &self.path,
            self.use_color,
            self.use_verbose,
            self.mask_passwords,
        );
        if self.use_color {
            crate::reset_color();
        }
        result
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
        path: &Stack<&String>,
        use_color: bool,
        use_verbose: bool,
        mask_passwords: bool,
    ) -> std::fmt::Result {
        let _ = match self {
            DiffResult::Identical { .. } => Ok(()),
            DiffResult::Changed { left, right } => {
                if use_color {
                    crate::set_fg(Some(Color::Red));
                }
                if use_verbose {
                    let indent = "  ".repeat(path.len());
                    write!(f, "- {}{}\n", indent, left)?;
                } else {
                    write!(
                        f,
                        "- {}\n",
                        path.append(&format!("{}", left)).mk_string("[", ", ", "]")
                    )?;
                }
                if use_color {
                    crate::set_fg(Some(Color::Green));
                }
                if use_verbose {
                    let indent = "  ".repeat(path.len());
                    write!(f, "+ {}{}\n", indent, right)
                } else {
                    write!(
                        f,
                        "+ {}\n",
                        path.append(&format!("{}", right)).mk_string("[", ", ", "]")
                    )
                }
            }
            DiffResult::InnerDifferences {
                left,
                inner_differences,
                ..
            } => {
                if use_verbose {
                    if use_color {
                        crate::set_fg(Some(Color::Yellow));
                    }
                    let indent = "  ".repeat(path.len());
                    write!(f, "~ {}{}\n", indent, left)?;
                }
                for id in inner_differences {
                    id.diff_result_format(
                        &mut f,
                        &path.append(&format!("{}", left)),
                        use_color,
                        use_verbose,
                        mask_passwords,
                    )?;
                }
                Ok(())
            }
            DiffResult::OnlyLeft { left } => {
                if use_color {
                    crate::set_fg(Some(Color::Red));
                }
                if use_verbose {
                    let indent = "  ".repeat(path.len());
                    write!(f, "- {}{}\n", indent, left)
                } else {
                    write!(
                        f,
                        "- {}\n",
                        path.append(&format!("{}", left)).mk_string("[", ", ", "]")
                    )
                }
            }
            DiffResult::OnlyRight { right } => {
                if use_color {
                    crate::set_fg(Some(Color::Green));
                }
                if use_verbose {
                    let indent = "  ".repeat(path.len());
                    write!(f, "+ {}{}\n", indent, right)
                } else {
                    write!(
                        f,
                        "+ {}\n",
                        path.append(&format!("{}", right)).mk_string("[", ", ", "]")
                    )
                }
            }
        };

        Ok(())
    }
}

pub fn diff_hashmap<'a, K, A, F>(
    a: &'a HashMap<K, A>,
    b: &'a HashMap<K, A>,
    order_fn: F,
) -> Vec<DiffResult<'a, A>>
where
    K: std::hash::Hash + Eq + Clone + std::fmt::Debug,
    A: Diff,
    F: Fn((&K, &A), (&K, &A)) -> std::cmp::Ordering,
{
    let mut keys: HashSet<K> = HashSet::new();
    keys.extend(a.keys().cloned());
    keys.extend(b.keys().cloned());

    // sort keys by order_fn, i.e. groups by names and entries by titles
    let mut keys: Vec<_> = keys.iter().collect();
    keys.sort_by(|k1, k2| {
        let v1 = a.get(*k1).or_else(|| b.get(*k1)).unwrap();
        let v2 = a.get(*k2).or_else(|| b.get(*k2)).unwrap();

        let ord = order_fn((k1, v1), (k2, v2));

        if ord == std::cmp::Ordering::Equal {
            // if the order_fn returns Equal, we can use the keys to break ties
            format!("{:?}", k1).cmp(&format!("{:?}", k2))
        } else {
            ord
        }
    });

    let mut acc: Vec<DiffResult<A>> = Vec::new();

    for key in keys {
        let Some(el_a): Option<&A> = a.get(&key) else {
            acc.push(DiffResult::OnlyRight {
                right: b.get(&key).unwrap(),
            });
            continue;
        };

        let Some(el_b): Option<&A> = b.get(&key) else {
            acc.push(DiffResult::OnlyLeft { left: el_a });
            continue;
        };

        let dr = el_a.diff(el_b);
        if let DiffResult::Identical { .. } = dr {
        } else {
            acc.push(dr);
        }
    }

    acc
}

#[cfg(test)]
mod test {

    use super::*;
    use diff::group::Group;

    #[test]
    fn diff_empty_groups() {
        let a = HashMap::<String, Group>::new();
        let b = HashMap::<String, Group>::new();
        let acc = diff_hashmap(&a, &b, |(_, a), (_, b)| a.name.cmp(&b.name));

        assert!(acc.is_empty());
    }
}
