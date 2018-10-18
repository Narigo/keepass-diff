extern crate rpassword;
extern crate termcolor;

mod diff;

use std::cmp::max;
use std::env;
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
    let a = diff::kdbx_to_sorted_vec(file_a, password_a);
    let b = diff::kdbx_to_sorted_vec(file_b, password_b);

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
