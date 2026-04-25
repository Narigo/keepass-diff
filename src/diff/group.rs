use crate::diff::entry::Entry;
use crate::diff::{Diff, DiffResult, DiffResultFormat};

use std::collections::HashMap;

/// Corresponds to a sorted Vec of KdbxEntry objects that can be diffed
#[derive(Debug)]
pub struct Group {
    pub name: String,
    pub child_groups: HashMap<keepass::db::GroupId, Group>,
    pub entries: HashMap<keepass::db::EntryId, Entry>,
    pub use_verbose: bool,
}

impl Group {
    /// Create an entries list from a keepass::Group
    pub fn from_keepass(
        group: keepass::db::GroupRef<'_>,
        use_verbose: bool,
        mask_passwords: bool,
    ) -> Self {
        let name = group.name.to_owned();

        let mut child_groups: HashMap<keepass::db::GroupId, Group> = HashMap::new();
        for child_group in group.groups() {
            child_groups.insert(
                child_group.id(),
                Group::from_keepass(child_group, use_verbose, mask_passwords),
            );
        }

        let mut entries: HashMap<keepass::db::EntryId, Entry> = HashMap::new();
        for entry in group.entries() {
            entries.insert(
                entry.id(),
                Entry::from_keepass(entry, use_verbose, mask_passwords),
            );
        }

        Group {
            name,
            child_groups,
            entries,
            use_verbose,
        }
    }
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.use_verbose {
            write!(f, "Group '{}'", self.name)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

/// Groups can be diffed.
impl Diff for Group {
    fn diff<'a>(&'a self, other: &'a Group) -> DiffResult<'a, Self> {
        let acc_groups =
            crate::diff::diff_hashmap(&self.child_groups, &other.child_groups, |(_, a), (_, b)| {
                a.name.cmp(&b.name)
            });

        let acc_entries =
            crate::diff::diff_hashmap(&self.entries, &other.entries, |(_, a), (_, b)| {
                let a_title = a
                    .fields
                    .get("Title")
                    .map(|f| f.value.as_str())
                    .unwrap_or("");
                let b_title = b
                    .fields
                    .get("Title")
                    .map(|f| f.value.as_str())
                    .unwrap_or("");

                a_title.cmp(b_title)
            });

        if acc_groups.is_empty() && acc_entries.is_empty() {
            return DiffResult::Identical {
                left: self,
                right: other,
            };
        } else {
            let mut inner_differences: Vec<Box<dyn DiffResultFormat>> = Vec::new();

            for dr in acc_groups {
                inner_differences.push(Box::new(dr));
            }

            for dr in acc_entries {
                inner_differences.push(Box::new(dr));
            }

            DiffResult::InnerDifferences {
                left: self,
                right: other,
                inner_differences,
            }
        }
    }
}
