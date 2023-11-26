use base64::{engine::general_purpose, Engine};
use keepass::db::Value;

use crate::diff::{Diff, DiffResult};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueType {
    Binary,
    Unprotected,
    Protected,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    name: String,
    value: String,
    kind: ValueType,
    use_verbose: bool,
    mask_passwords: bool,
}

impl Field {
    pub fn from_keepass(
        name: String,
        value: &Value,
        use_verbose: bool,
        mask_passwords: bool,
    ) -> Self {
        Field {
            name,
            value: match value {
                Value::Bytes(b) => general_purpose::STANDARD_NO_PAD.encode(b),
                Value::Unprotected(v) => v.to_owned(),
                Value::Protected(p) => String::from_utf8(p.unsecure().to_owned())
                    .unwrap()
                    .to_owned(),
            },
            kind: match value {
                Value::Bytes(_) => ValueType::Binary,
                Value::Unprotected(_) => ValueType::Unprotected,
                Value::Protected(_) => ValueType::Protected,
            },
            use_verbose,
            mask_passwords,
        }
    }

    pub fn value(&self) -> &str {
        self.value.as_str()
    }
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
        let password_str = match (self.mask_passwords, self.kind) {
            (true, ValueType::Protected) => "***",
            _ => self.value.as_str(),
        };
        if self.use_verbose {
            write!(f, "Field '{}' = '{}'", self.name, password_str)
        } else {
            write!(f, "{} = {}", self.name, password_str)
        }
    }
}
