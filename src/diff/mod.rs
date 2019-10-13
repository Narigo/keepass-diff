use termcolor::Color;

pub mod entry;
pub mod group;

/// The possible outcomes of diffing two objects against another
#[derive(Debug)]
pub enum DiffResult<'a, T> {
    /// The objects are identical, including any children
    Identical { left: &'a T, right: &'a T },
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
        depth: usize,
        use_color: bool,
    ) -> std::fmt::Result;
}

/// Helper wrapper to impl Display for a DiffResult with user-specified settings
pub struct DiffDisplay<T: DiffResultFormat> {
    pub inner: T,
    pub depth: usize,
    pub use_color: bool,
}

impl<T: DiffResultFormat> std::fmt::Display for DiffDisplay<T> {
    fn fmt(&self, mut f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner
            .diff_result_format(&mut f, self.depth, self.use_color)
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
        depth: usize,
        use_color: bool,
    ) -> std::fmt::Result {
        let indent = "  ".repeat(depth);
        match self {
            DiffResult::Identical { .. } => Ok(()),
            DiffResult::InnerDifferences {
                left,
                inner_differences,
                ..
            } => {
                if use_color {
                    crate::set_fg(Some(Color::Yellow));
                }
                write!(f, "{}~ {}\n", indent, left)?;
                for id in inner_differences {
                    id.diff_result_format(&mut f, depth + 1, use_color)?;
                }
                write!(f, "\n")
            }
            DiffResult::OnlyLeft { left } => {
                if use_color {
                    crate::set_fg(Some(Color::Red));
                }
                write!(f, "{}- {}\n", indent, left)
            }
            DiffResult::OnlyRight { right } => {
                if use_color {
                    crate::set_fg(Some(Color::Green));
                }
                write!(f, "{}+ {}\n", indent, right)
            }
        }?;

        if use_color {
            crate::set_fg(None);
        }

        Ok(())
    }
}
