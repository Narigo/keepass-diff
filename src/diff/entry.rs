use std::collections::HashMap;

use crate::diff::field::Field;
use crate::diff::{Diff, DiffResult, DiffResultFormat};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub fields: HashMap<String, Field>,
    use_verbose: bool,
    mask_passwords: bool,
}

impl Entry {
    pub fn from_keepass(
        e: keepass::db::EntryRef<'_>,
        use_verbose: bool,
        mask_passwords: bool,
    ) -> Self {
        // username, password, etc. are just fields
        let fields = e
            .fields
            .iter()
            .map(|(k, v)| {
                (
                    k.to_owned(),
                    Field {
                        name: k.to_owned(),
                        value: v.get().to_string(),
                        protected: v.is_protected(),
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
        let field_differences =
            crate::diff::diff_hashmap(&self.fields, &other.fields, |(ka, _), (kb, _)| ka.cmp(&kb));

        if field_differences.is_empty() {
            DiffResult::Identical {
                left: self,
                right: other,
            }
        } else {
            let mut inner_differences: Vec<Box<dyn DiffResultFormat>> = Vec::new();

            for dr in field_differences {
                inner_differences.push(Box::new(dr))
            }

            DiffResult::InnerDifferences {
                left: self,
                right: other,
                inner_differences,
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
                protected: false,
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
