extern crate keepass;
extern crate rpassword;
extern crate termcolor;

use keepass::{Database, Group, OpenDBError};
use std::cmp::max;
use std::env;
use std::fs::File;
use std::path::Path;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    let args: Vec<String> = env::args().collect();

    match (args.get(1), args.get(2)) {
        (Some(file_a), Some(file_b)) => {
            print!("Password for file {}: ", file_a);
            let pass_a = rpassword::prompt_password_stdout("").unwrap();
            print!("Password for file {}: ", file_b);
            let pass_b = rpassword::prompt_password_stdout("").unwrap();
            compare(
                &file_a.as_str(),
                &pass_a.as_str(),
                &file_b.as_str(),
                &pass_b.as_str(),
            )
        }
        _ => println!("Need two .kdbx files as arguments"),
    }
}

fn compare(file_a: &str, password_a: &str, file_b: &str, password_b: &str) {
    let a = kdbx_to_sorted_vec(file_a, password_a);
    let b = kdbx_to_sorted_vec(file_b, password_b);

    let maximum = max(a.len(), b.len());
    let mut a_idx = 0;
    let mut b_idx = 0;

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    while a_idx < maximum && b_idx < maximum {
        let a_elem = a.get(a_idx);
        let b_elem = b.get(b_idx);
        if a_elem == b_elem {
            a_idx = a_idx + 1;
            b_idx = b_idx + 1;
            continue;
        }
        if a_elem < b_elem {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                .unwrap();
            println!("- {:?}", a_elem.unwrap());
            a_idx = a_idx + 1;
            continue;
        }
        if b_elem < a_elem {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                .unwrap();
            println!("+ {:?}", b_elem.unwrap());
            b_idx = b_idx + 1;
            continue;
        }
    }
    stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();
}

fn kdbx_to_sorted_vec(
    file: &str,
    password: &str,
) -> Vec<(Vec<String>, Option<String>, Option<String>, Option<String>)> {
    let db = File::open(Path::new(file))
        .map_err(|e| OpenDBError::from(e))
        .and_then(|mut db_file| Database::open(&mut db_file, password))
        .unwrap();

    accumulate_all_entries(db.root)
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
    let mut all_groups_children: Vec<(
        Vec<String>,
        Option<String>,
        Option<String>,
        Option<String>,
    )> = Vec::new();
    for next_parent in current_group.child_groups {
        let children = check_group(&mut accumulated.clone(), &mut parents.clone(), next_parent);
        all_groups_children.append(&mut children.clone())
    }
    accumulated.append(&mut all_groups_children);
    accumulated.clone()
}
