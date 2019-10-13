use keepass::Value;
use std::collections::HashMap;

use crate::diff::{Diff, DiffResult};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub fields: HashMap<String, String>,
}

impl Entry {
    pub fn from_keepass(e: &keepass::Entry) -> Self {
        // username, password, etc. are just fields
        let fields = e
            .fields
            .iter()
            .map(|(k, v)| {
                (
                    k.to_owned(),
                    match v {
                        Value::Unprotected(v) => v.to_owned(),
                        Value::Protected(p) => String::from_utf8(p.unsecure().to_owned())
                            .unwrap()
                            .to_owned(),
                    },
                )
            })
            .collect();

        Entry { fields }
    }
}

impl Diff for Entry {
    fn diff<'a>(&'a self, other: &'a Self) -> DiffResult<'a, Self> {
        // TODO actually compare field contents
        DiffResult::Identical {
            left: self,
            right: other,
        }
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Entry '{}'", self.fields.get("Title").unwrap())
    }
}
