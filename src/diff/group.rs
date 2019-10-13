use crate::diff::entry::Entry;
use crate::diff::{Diff, DiffResult, DiffResultFormat};

use std::collections::HashMap;

/// Corresponds to a sorted Vec of KdbxEntry objects that can be diffed
#[derive(Debug)]
pub struct Group {
    name: String,
    child_groups: HashMap<String, Group>,
    entries: HashMap<String, Entry>,
}

impl Group {
    /// Create an entries list from a keepass::Group
    pub fn from_keepass(group: &keepass::Group) -> Self {
        let child_groups = group
            .child_groups
            .iter()
            .map(|(k, v)| (k.clone(), Group::from_keepass(&v)))
            .collect();

        let entries = group
            .entries
            .iter()
            .map(|(k, v)| (k.clone(), Entry::from_keepass(&v)))
            .collect();

        Group {
            name: group.name.to_owned(),
            child_groups,
            entries,
        }
    }
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Group '{}'", self.name)
    }
}

/// Groups can be diffed.
impl Diff for Group {
    fn diff<'a>(&'a self, other: &'a Group) -> DiffResult<'a, Self> {
        let (has_differences_groups, acc_groups) =
            crate::diff::diff_hashmap(&self.child_groups, &other.child_groups);

        let (has_differences_entries, acc_entries) =
            crate::diff::diff_hashmap(&self.entries, &other.entries);

        if has_differences_groups || has_differences_entries {
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
        } else {
            DiffResult::Identical {
                left: self,
                right: other,
            }
        }
    }
}
