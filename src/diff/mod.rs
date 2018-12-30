extern crate keepass;

use self::keepass::{Database, Group, OpenDBError};
use std::fs::File;
use std::path::Path;

pub fn kdbx_to_sorted_vec(
  file: &str,
  password: &str,
) -> Result<Vec<(Vec<String>, Option<String>, Option<String>, Option<String>)>, &'static str> {
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
) -> Vec<(Vec<String>, Option<String>, Option<String>, Option<String>)> {
  let mut accumulated = check_group(&mut Vec::new(), &mut Vec::new(), start);
  accumulated.sort();
  accumulated.dedup();
  accumulated
}

fn check_group(
  accumulated: &mut Vec<(Vec<String>, Option<String>, Option<String>, Option<String>)>,
  parents: &mut Vec<String>,
  current_group: Group,
) -> Vec<(Vec<String>, Option<String>, Option<String>, Option<String>)> {
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
    Vec::<(Vec<String>, Option<String>, Option<String>, Option<String>)>::new();
  for next_parent in current_group.child_groups {
    let children = check_group(&mut accumulated.clone(), &mut parents.clone(), next_parent);
    all_groups_children.append(&mut children.clone())
  }
  accumulated.append(&mut all_groups_children);
  accumulated.clone()
}
