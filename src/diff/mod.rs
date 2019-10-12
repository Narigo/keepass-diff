pub mod group;

/// The possible outcomes of diffing two objects against another
#[derive(Debug)]
pub enum DiffResult<'a, T, I> {
    /// The objects are identical, including any children
    Identical { left: &'a T, right: &'a T },
    /// There is a difference in a child object
    InnerDifferences {
        left: &'a T,
        right: &'a T,
        inner_differences: Vec<I>,
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
    type Inner;
    type InnerInner;
    fn diff<'a>(
        &'a self,
        other: &'a Self,
    ) -> DiffResult<'a, Self, DiffResult<'a, Self::Inner, Self::InnerInner>>;
}

/// Denotes that an object can be formatted as a DiffResult
pub trait DiffResultFormat {
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
