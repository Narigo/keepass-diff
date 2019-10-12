use keepass::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KdbxEntry {
    pub path: Vec<String>,
    pub fields: HashMap<String, String>,
}

impl KdbxEntry {
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

        KdbxEntry { path, fields }
    }

    pub fn get_title(&self) -> &str {
        self.path.get(self.path.len() - 1).unwrap()
    }
}

/// Sort KdbxEntry by their paths
impl std::cmp::Ord for KdbxEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.path.cmp(&other.path)
    }
}

/// Sort KdbxEntry by their paths
impl std::cmp::PartialOrd for KdbxEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
