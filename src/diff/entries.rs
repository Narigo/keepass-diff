use crate::diff::{Diff, DiffResult};
use crate::entry::KdbxEntry;

use keepass::Group;

use std::cmp::max;

/// Corresponds to a sorted Vec of KdbxEntry objects that can be diffed
#[derive(Debug)]
pub struct Entries {
    entries: Vec<KdbxEntry>,
}

impl Entries {
    /// Create an entries list from a keepass::Group
    pub fn from_keepass(root: keepass::Group) -> Self {
        let mut entries = Vec::new();
        check_group(&mut entries, &Vec::new(), &root);

        entries.sort();
        entries.dedup();

        Entries { entries }
    }
}

impl Diff for Entries {
    type Inner = KdbxEntry;
    type InnerInner = ();
    fn diff<'a>(
        &'a self,
        other: &'a Entries,
    ) -> DiffResult<'a, Self, DiffResult<'a, Self::Inner, Self::InnerInner>> {
        let left = &self.entries;
        let right = &other.entries;

        let maximum = max(left.len(), right.len());

        let mut left_idx = 0;
        let mut right_idx = 0;

        let mut acc = Vec::new();

        let mut has_differences = false;

        // keep going until the indices both point to the end
        while left_idx < maximum && right_idx < maximum {
            let left_elem = left.get(left_idx);
            let right_elem = right.get(right_idx);
            match (left_elem, right_elem) {
                (Some(a), Some(b)) => {
                    if a < b {
                        left_idx = left_idx + 1;
                        acc.push(DiffResult::OnlyLeft { left: a });
                        has_differences = true;
                    } else if b < a {
                        right_idx = right_idx + 1;
                        acc.push(DiffResult::OnlyRight { right: b });
                        has_differences = true;
                    } else {
                        left_idx = left_idx + 1;
                        right_idx = right_idx + 1;
                        acc.push(DiffResult::Identical { left: a, right: b });
                    }
                }
                (Some(a), None) => {
                    left_idx = left_idx + 1;
                    acc.push(DiffResult::OnlyLeft { left: a });
                    has_differences = true;
                }
                (None, Some(b)) => {
                    right_idx = right_idx + 1;
                    acc.push(DiffResult::OnlyRight { right: b });
                    has_differences = true;
                }
                (None, None) => {
                    break;
                }
            }
        }

        if has_differences {
            DiffResult::InnerDifferences {
                inner_differences: acc,
            }
        } else {
            DiffResult::Identical {
                left: self,
                right: other,
            }
        }
    }
}

/// Recursively add all entries from current_group and its children to the accumulated Vec
fn check_group(
    mut accumulated: &mut Vec<KdbxEntry>,
    parents: &Vec<String>,
    current_group: &Group,
) -> Vec<KdbxEntry> {
    // make new path containing current group name
    let mut parents = parents.clone();
    parents.push(current_group.name.to_owned());

    // add all entries
    for (_, entry) in &current_group.entries {
        accumulated.push(KdbxEntry::from_keepass(&parents, &entry));
    }

    // recursively get all children
    for (_, group) in &current_group.child_groups {
        check_group(&mut accumulated, &parents, &group);
    }
    accumulated.clone()
}
