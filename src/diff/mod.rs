pub mod entries;

#[derive(Debug)]
pub enum DiffResult<'a, T, I> {
    Identical { left: &'a T, right: &'a T },
    InnerDifferences { inner_differences: Vec<I> },
    OnlyLeft { left: &'a T },
    OnlyRight { right: &'a T },
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
