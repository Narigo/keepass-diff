use crate::diff::{Diff, DiffResult};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub protected: bool,
    pub use_verbose: bool,
    pub mask_passwords: bool,
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
        let value = if self.mask_passwords && self.protected {
            "***"
        } else {
            &self.value
        };

        if self.use_verbose {
            write!(f, "Field '{}' = '{}'", self.name, value)
        } else {
            write!(f, "{} = {}", self.name, value)
        }
    }
}
