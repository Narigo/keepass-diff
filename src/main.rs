extern crate clap;
extern crate rpassword;
extern crate termcolor;

mod diff;

use clap::{App, Arg};
use std::cmp::max;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    let matches = App::new("keepass-diff")
        .version("0.1.0")
        .about("Shows differences between two .kdbx files")
        .author("Joern Bernhardt")
        .arg(
            Arg::with_name("INPUT_A")
                .help("Sets the first file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("INPUT_B")
                .help("Sets the second file")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("password-a")
                .long("password-a")
                .help("Sets the password for the first file (will be asked for if omitted)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("password-b")
                .long("password-b")
                .help("Sets the password for the second file (will be asked for if omitted)")
                .takes_value(true),
        )
        .get_matches();

    match (matches.value_of("INPUT_A"), matches.value_of("INPUT_B")) {
        (Some(file_a), Some(file_b)) => {
            let pass_a = match matches.value_of("password-a") {
                Some(password) => password.to_string(),
                _ => {
                    print!("Password for file {}: ", file_a);
                    rpassword::prompt_password_stdout("").unwrap()
                }
            };
            let pass_b = match matches.value_of("password-b") {
                Some(password) => password.to_string(),
                _ => {
                    print!("Password for file {}: ", file_b);
                    rpassword::prompt_password_stdout("").unwrap()
                }
            };
            compare(&file_a, &pass_a, &file_b, &pass_b)
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
