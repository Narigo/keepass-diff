use base64::{engine::general_purpose, Engine as _};
use keepass::db::Value;
use std::collections::HashMap;

use crate::diff::field::{Field, ValueType};
use crate::diff::{Diff, DiffResult, DiffResultFormat};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub fields: HashMap<String, Field>,
    use_verbose: bool,
    mask_passwords: bool,
}

impl Entry {
    pub fn from_keepass(e: &keepass::db::Entry, use_verbose: bool, mask_passwords: bool) -> Self {
        // username, password, etc. are just fields
        let fields = e
            .fields
            .iter()
            .map(|(k, v)| {
                (
                    k.to_owned(),
                    Field {
                        name: k.to_owned(),
                        value: match v {
                            Value::Bytes(b) => general_purpose::STANDARD_NO_PAD.encode(b),
                            Value::Unprotected(v) => v.to_owned(),
                            Value::Protected(p) => String::from_utf8(p.unsecure().to_owned())
                                .unwrap()
                                .to_owned(),
                        },
                        kind: match v {
                            Value::Bytes(_) => ValueType::Binary,
                            Value::Unprotected(_) => ValueType::Unprotected,
                            Value::Protected(_) => ValueType::Protected,
                        },
                        use_verbose,
                        mask_passwords,
                    },
                )
            })
            .collect();

        Entry {
            fields,
            use_verbose,
            mask_passwords,
        }
    }
}

impl Diff for Entry {
    fn diff<'a>(&'a self, other: &'a Self) -> DiffResult<'a, Self> {
        let (has_differences, field_differences) =
            crate::diff::diff_entry(&self.fields, &other.fields);

        if has_differences {
            let mut inner_differences: Vec<Box<dyn DiffResultFormat>> = Vec::new();

            for dr in field_differences {
                inner_differences.push(Box::new(dr))
            }

            DiffResult::InnerDifferences {
                left: self,
                right: other,
                inner_differences,
            }
        } else {
            DiffResult::Identical {
                left: self,
                right: other,
            }
        }
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = self
            .fields
            .get("Title")
            .unwrap_or(&Field {
                name: "Title".to_string(),
                value: "".to_string(),
                kind: ValueType::Unprotected,
                use_verbose: self.use_verbose,
                mask_passwords: self.mask_passwords,
            })
            .value
            .clone();
        if self.use_verbose {
            write!(f, "Entry '{}'", name)
        } else {
            write!(f, "{}", name)
        }
    }
}
