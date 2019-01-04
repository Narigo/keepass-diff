extern crate keepass;

use self::keepass::{Database, Group, OpenDBError};
use std::cmp::max;
use std::fs::File;
use std::path::Path;

type KdbxEntry = (Vec<String>, Option<String>, Option<String>, Option<String>);
type SortedKdbxEntries = Vec<KdbxEntry>;

pub enum ComparedEntry<T> {
  Both(T),
  OnlyLeft(T),
  OnlyRight(T),
}

pub fn compare(left: SortedKdbxEntries, right: SortedKdbxEntries) -> Vec<ComparedEntry<KdbxEntry>> {
  let maximum = max(left.len(), right.len());
  let mut left_idx = 0;
  let mut right_idx = 0;

  let mut acc = Vec::<ComparedEntry<KdbxEntry>>::new();
  while left_idx < maximum && right_idx < maximum {
    let left_elem = left.get(left_idx);
    let right_elem = right.get(right_idx);
    if left_elem == right_elem {
      left_idx = left_idx + 1;
      right_idx = right_idx + 1;
      acc.push(ComparedEntry::Both(left_elem.unwrap().clone()));
      continue;
    }
    if left_elem < right_elem {
      acc.push(ComparedEntry::OnlyLeft(left_elem.unwrap().clone()));
      left_idx = left_idx + 1;
      continue;
    }
    if right_elem < left_elem {
      acc.push(ComparedEntry::OnlyRight(right_elem.unwrap().clone()));
      right_idx = right_idx + 1;
      continue;
    }
  }
  acc
}

pub fn kdbx_to_sorted_vec(
  file: &str,
  password: &str,
) -> Result<SortedKdbxEntries, &'static str> {
  File::open(Path::new(file))
    .map_err(|e| OpenDBError::from(e))
    .and_then(|mut db_file| Database::open(&mut db_file, password))
    .map(|db: Database| accumulate_all_entries(db.root))
    .map_err(|e: OpenDBError| match e {
      OpenDBError::Crypto(_) => "Decryption error",
      _ => "unknown error",
    })
}

fn accumulate_all_entries(
  start: Group,
) -> SortedKdbxEntries {
  let mut accumulated = check_group(&mut Vec::new(), &mut Vec::new(), start);
  accumulated.sort();
  accumulated.dedup();
  accumulated
}

fn check_group(
  accumulated: &mut Vec<KdbxEntry>,
  parents: &mut Vec<String>,
  current_group: Group,
) -> Vec<KdbxEntry> {
  parents.push(current_group.name);
  for entry in current_group.entries {
    accumulated.push((
      parents.clone(),
      entry.get_title().map(|x| x.to_string()),
      entry.get_username().map(|x| x.to_string()),
      entry.get_password().map(|x| x.to_string()),
    ))
  }
  let mut all_groups_children =
    Vec::<KdbxEntry>::new();
  for next_parent in current_group.child_groups {
    let children = check_group(&mut accumulated.clone(), &mut parents.clone(), next_parent);
    all_groups_children.append(&mut children.clone())
  }
  accumulated.append(&mut all_groups_children);
  accumulated.clone()
}
