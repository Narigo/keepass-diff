extern crate keepass;

use self::keepass::{result::Error, result::Result, Database, Group};
use std::cmp::max;
use std::path::Path;
use std::{fs::File, io::Read};

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
  println!("maximum {}", maximum);
  while left_idx < left.len() && right_idx < right.len() {
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
      println!("right_idx {}, left_idx {}", right_idx, left_idx);
      match right_elem {
        Some(x) => println!("got a right_elem {:?}", x),
        None => println!("no right elem??"),
      }
      acc.push(ComparedEntry::OnlyRight(right_elem.unwrap().clone()));
      right_idx = right_idx + 1;
      continue;
    }
  }
  acc
}

pub fn kdbx_to_sorted_vec(
  file: &str,
  password: Option<String>,
  keyfile_path: Option<&str>,
) -> Result<SortedKdbxEntries> {
  println!("in kdbx_to_sorted_vec");
  let mut keyfile = keyfile_path.map(|path| File::open(Path::new(path)).unwrap());
  println!("has a keyfile path? {}", keyfile_path.is_some());
  File::open(Path::new(file))
    .map_err(|e| {
      println!("error opening file {}", file);
      Error::from(e)
    })
    .and_then(|mut db_file| {
      println!("got a db_file!");
      let db = Database::open(
        &mut db_file,
        password.as_ref().map(|s| s.as_str()),
        keyfile.as_mut().map(|f| f as &mut dyn Read),
      );
      println!("opened the db");
      db
    })
    .map(|db: Database| {
      println!("accumulate_all_entries will be called!");
      accumulate_all_entries(db.root)
    })
}

fn accumulate_all_entries(start: Group) -> SortedKdbxEntries {
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
  for (_, entry) in current_group.entries {
    accumulated.push((
      parents.clone(),
      entry.get_title().map(|x| x.to_string()),
      entry.get_username().map(|x| x.to_string()),
      entry.get_password().map(|x| x.to_string()),
    ))
  }
  let mut all_groups_children = Vec::<KdbxEntry>::new();
  for (_, next_parent) in current_group.child_groups {
    let children = check_group(&mut accumulated.clone(), &mut parents.clone(), next_parent);
    all_groups_children.append(&mut children.clone())
  }
  accumulated.append(&mut all_groups_children);
  accumulated.clone()
}
