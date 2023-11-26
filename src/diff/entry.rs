use std::collections::HashMap;

use crate::diff::field::Field;
use crate::diff::{Diff, DiffResult, DiffResultFormat};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    fields: HashMap<String, Field>,
    use_verbose: bool,
    mask_passwords: bool,
}

impl Entry {
    pub fn from_keepass(
        entry: &keepass::db::Entry,
        use_verbose: bool,
        mask_passwords: bool,
    ) -> Self {
        // username, password, etc. are just fields
        let fields = entry
            .fields
            .iter()
            .map(|(key, value)| {
                (
                    key.to_owned(),
                    Field::from_keepass(key.to_owned(), value, use_verbose, mask_passwords),
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
            let inner_differences = field_differences
                .into_iter()
                .map(|dr| Box::new(dr) as Box<dyn DiffResultFormat>)
                .collect();

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
        let name = self.fields.get("Title").map(|f| f.value()).unwrap_or("");
        if self.use_verbose {
            write!(f, "Entry '{}'", name)
        } else {
            write!(f, "{}", name)
        }
    }
}
