extern crate clap;
extern crate rpassword;
extern crate termcolor;

mod diff;

use clap::{App, Arg};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
  let matches = App::new("keepass-diff")
    .version("0.1.0")
    .about("Shows differences between two .kdbx files")
    .author("Joern Bernhardt")
    .arg(
      Arg::with_name("INPUT-A")
        .help("Sets the first file")
        .required(true)
        .index(1),
    ).arg(
      Arg::with_name("INPUT-B")
        .help("Sets the second file")
        .required(true)
        .index(2),
    ).arg(
      Arg::with_name("no-color")
        .short("C")
        .long("no-color")
        .help("Disables color output")
        .takes_value(false),
    ).arg(
      Arg::with_name("password-a")
        .long("password-a")
        .help("Sets the password for the first file (will be asked for if omitted)")
        .takes_value(true),
    ).arg(
      Arg::with_name("password-b")
        .long("password-b")
        .help("Sets the password for the second file (will be asked for if omitted)")
        .takes_value(true),
    ).arg(
      Arg::with_name("passwords")
        .long("passwords")
        .help("Sets the password for both files (if it's the same for both files)")
        .takes_value(true),
    ).get_matches();

  match (matches.value_of("INPUT-A"), matches.value_of("INPUT-B")) {
    (Some(file_a), Some(file_b)) => {
      let pass_a = match (
        matches.value_of("password-a"),
        matches.value_of("passwords"),
      ) {
        (Some(password), _) => password.to_string(),
        (_, Some(password)) => password.to_string(),
        _ => {
          print!("Password for file {}: ", file_a);
          rpassword::prompt_password_stdout("").unwrap()
        }
      };
      let pass_b = match (
        matches.value_of("password-b"),
        matches.value_of("passwords"),
      ) {
        (Some(password), _) => password.to_string(),
        (_, Some(password)) => password.to_string(),
        _ => {
          print!("Password for file {}: ", file_b);
          rpassword::prompt_password_stdout("").unwrap()
        }
      };
      let no_color: bool = matches.is_present("no-color");
      compare(&file_a, &pass_a, &file_b, &pass_b, !no_color)
    }
    _ => println!("Need two .kdbx files as arguments"),
  }
}

fn compare(file_a: &str, password_a: &str, file_b: &str, password_b: &str, use_color: bool) {
  diff::kdbx_to_sorted_vec(file_a, password_a)
    .and_then(|a| diff::kdbx_to_sorted_vec(file_b, password_b).map(|b| (a, b)))
    .map(|r| match r {
      (a, b) => diff::compare(a, b),
    }).map(|r| {
      let mut i = 0;
      let mut stdout = StandardStream::stdout(ColorChoice::Always);
      while i < r.len() {
        match r.get(i) {
          Some(diff::ComparedEntry::OnlyLeft(elem)) => {
            if use_color {
              stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                .unwrap();
            }
            println!("- {:?}", elem)
          }
          Some(diff::ComparedEntry::OnlyRight(elem)) => {
            if use_color {
              stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                .unwrap();
            }
            println!("+ {:?}", elem)
          }
          _ => {}
        }
        i = i + 1;
      }
      if use_color {
        stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();
      }
    }).unwrap_or_else(|err| println!("{}", err));
}
