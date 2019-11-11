use crate::diff::{Diff, DiffResult};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub use_verbose: bool,
}

impl Diff for Field {
    fn diff<'a>(&'a self, other: &'a Self) -> DiffResult<'a, Self> {
        if self.value == other.value {
            DiffResult::Identical {
                left: self,
                right: other,
            }
        } else {
            DiffResult::Changed {
                left: self,
                right: other,
            }
        }
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.use_verbose {
            write!(f, "Field '{}' = '{}'", self.name, self.value)
        } else {
            write!(f, "{} = {}", self.name, self.value)
        }
    }
}
