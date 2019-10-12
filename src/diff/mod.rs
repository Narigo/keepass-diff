pub mod entries;

#[derive(Debug)]
pub enum DiffResult<'a, T, I> {
    Identical {
        left: &'a T,
        right: &'a T,
    },
    InnerDifferences {
        left: &'a T,
        right: &'a T,
        inner_differences: Vec<I>,
    },
    OnlyLeft {
        left: &'a T,
    },
    OnlyRight {
        right: &'a T,
    },
}

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

pub trait DiffResultFormat {
    fn diff_result_format(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        depth: usize,
        use_color: bool,
    ) -> std::fmt::Result;
}

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
