use crate::diff::entry::Entry;
use crate::diff::{Diff, DiffResult, DiffResultFormat};

use std::collections::{HashMap, HashSet};

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

/// Compare to HashMaps of the same value type with each other, returning a bool indicating whether
/// there are any differences and a Vec<DiffResult<A>> listing all differences
pub fn diff_hashmap<'a, A>(
    a: &'a HashMap<String, A>,
    b: &'a HashMap<String, A>,
) -> (bool, Vec<DiffResult<'a, A>>)
where
    A: Diff,
{
    let mut keys = HashSet::new();
    keys.extend(a.keys());
    keys.extend(b.keys());

    let mut keys: Vec<_> = keys.iter().collect();
    keys.sort();

    let mut acc: Vec<DiffResult<A>> = Vec::new();

    let mut has_differences = false;

    for key in keys {
        let el_a: Option<&A> = a.get(*key);
        let el_b: Option<&A> = b.get(*key);

        match (el_a, el_b) {
            // both a and b have the key
            (Some(v_a), Some(v_b)) => {
                let dr: DiffResult<A> = v_a.diff(v_b);

                if let DiffResult::Identical { .. } = dr {
                } else {
                    has_differences = true;
                }

                acc.push(dr);
            }

            // only a has the key
            (Some(v_a), None) => {
                has_differences = true;
                acc.push(DiffResult::OnlyLeft { left: v_a })
            }

            // only b has the key
            (None, Some(v_b)) => {
                has_differences = true;
                acc.push(DiffResult::OnlyRight { right: v_b })
            }

            // none have the key (this shouldn't happen)
            (None, None) => {}
        }
    }

    (has_differences, acc)
}

/// Groups can be diffed.
impl Diff for Group {
    fn diff<'a>(&'a self, other: &'a Group) -> DiffResult<'a, Self> {
        let (hd_groups, acc_groups) = diff_hashmap(&self.child_groups, &other.child_groups);
        let (hd_entries, acc_entries) = diff_hashmap(&self.entries, &other.entries);

        if hd_groups || hd_entries {
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
