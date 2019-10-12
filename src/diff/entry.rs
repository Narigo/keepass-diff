use keepass::Value;
use std::collections::HashMap;
use termcolor::Color;

use crate::diff::{DiffResult, DiffResultFormat};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub path: Vec<String>,
    pub fields: HashMap<String, String>,
}

impl Entry {
    pub fn from_keepass(parents: &Vec<String>, e: &keepass::Entry) -> Self {
        // add current title to path
        let mut path = parents.clone();
        path.push(e.get_title().unwrap().to_owned());

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

        Entry { path, fields }
    }

    pub fn get_title(&self) -> &str {
        self.path.get(self.path.len() - 1).unwrap()
    }
}

/// Sort KdbxEntry by their paths
impl std::cmp::Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.path.cmp(&other.path)
    }
}

/// Sort KdbxEntry by their paths
impl std::cmp::PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> DiffResultFormat for DiffResult<'a, Entry, ()> {
    fn diff_result_format(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        depth: usize,
        use_color: bool,
    ) -> std::fmt::Result {
        let indent = "  ".repeat(depth);
        match self {
            DiffResult::Identical { .. } => {
                write!(f, "")?;
            }
            DiffResult::InnerDifferences { left, .. } => {
                // TODO recursively list differences within entries
                if use_color {
                    crate::set_fg(Some(Color::Yellow));
                }
                write!(f, "{}~ Entry '{}'\n", indent, left.get_title())?;
            }
            DiffResult::OnlyLeft { left } => {
                if use_color {
                    crate::set_fg(Some(Color::Red));
                }
                write!(f, "{}- Entry '{}'\n", indent, left.get_title())?;
            }
            DiffResult::OnlyRight { right } => {
                if use_color {
                    crate::set_fg(Some(Color::Green));
                }
                write!(f, "{}+ Entry '{}'\n", indent, right.get_title())?;
            }
        }

        if use_color {
            crate::set_fg(None);
        }

        Ok(())
    }
}
