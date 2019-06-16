extern crate clap;
extern crate keepass;
extern crate rpassword;
extern crate termcolor;

mod apply;
mod diff;

use apply::apply;
use clap::{App, Arg};
use diff::{
  compare, kdbx_to_sorted_vec,
  ComparedEntry::{OnlyLeft, OnlyRight},
};
use keepass::result::Result;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() -> Result<()> {
  let matches = App::new("keepass-diff")
    .version("0.3.0")
    .about("Shows differences between two .kdbx files")
    .author("Joern Bernhardt")
    .arg(
      Arg::with_name("INPUT-A")
        .help("Sets the first file")
        .required(true)
        .index(1),
    )
    .arg(
      Arg::with_name("INPUT-B")
        .help("Sets the second file")
        .required(true)
        .index(2),
    )
    .arg(
      Arg::with_name("no-color")
        .short("C")
        .long("no-color")
        .help("Disables color output")
        .takes_value(false),
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
    .arg(
      Arg::with_name("passwords")
        .long("passwords")
        .help("Sets the password for both files (if it's the same for both files)")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("no-password-a")
        .long("no-password-a")
        .help("Sets no password for the first file (and will not ask for it)")
        .takes_value(false),
    )
    .arg(
      Arg::with_name("no-password-b")
        .long("no-password-b")
        .help("Sets no password for the second file (and will not ask for it)")
        .takes_value(false),
    )
    .arg(
      Arg::with_name("no-passwords")
        .long("no-passwords")
        .help("Sets no password for both files (and will not ask for both files)")
        .takes_value(false),
    )
    .arg(
      Arg::with_name("keyfile-a")
        .long("keyfile-a")
        .help("Sets the key file for the first file")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("keyfile-b")
        .long("keyfile-b")
        .help("Sets the key file for the second file")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("keyfiles")
        .long("keyfiles")
        .help("Sets the same key file for both files (keyfile-a and keyfile-b would take precedence if set as well)")
        .takes_value(true),
    )
    .get_matches();

  match (matches.value_of("INPUT-A"), matches.value_of("INPUT-B")) {
    (Some(file_a), Some(file_b)) => {
      let pass_a = match (
        matches.value_of("password-a"),
        matches.value_of("passwords"),
        matches.is_present("no-password-a"),
        matches.is_present("no-passwords"),
      ) {
        (Some(password), _, _, _) => Some(String::from(password)),
        (_, Some(password), _, _) => Some(String::from(password)),
        (_, _, true, _) => None,
        (_, _, _, true) => None,
        _ => {
          print!("Password for file {}: ", file_a);
          let password = rpassword::prompt_password_stdout("")
            .map(|s| if s == "" { None } else { Some(s) })
            .unwrap_or(None);
          password
        }
      };
      let pass_b = match (
        matches.value_of("password-b"),
        matches.value_of("passwords"),
        matches.is_present("no-password-b"),
        matches.is_present("no-passwords"),
      ) {
        (Some(password), _, _, _) => Some(String::from(password)),
        (_, Some(password), _, _) => Some(String::from(password)),
        (_, _, true, _) => None,
        (_, _, _, true) => None,
        _ => {
          print!("Password for file {}: ", file_b);
          let password_option: Option<String> = rpassword::prompt_password_stdout("")
            .map(|s| if s == "" { None } else { Some(s) })
            .unwrap_or(None);
          password_option
        }
      };
      let keyfile_a: Option<&str> = matches
        .value_of("keyfile-a")
        .or(matches.value_of("keyfiles"));
      let keyfile_b: Option<&str> = matches
        .value_of("keyfile-b")
        .or(matches.value_of("keyfiles"));
      let no_color: bool = matches.is_present("no-color");
      run_comparison(
        &file_a, pass_a, &file_b, pass_b, !no_color, keyfile_a, keyfile_b,
      )
    }
    _ => println!("Need two .kdbx files as arguments"),
  }

  Ok(())
}

fn run_comparison(
  file_a: &str,
  password_a: Option<String>,
  file_b: &str,
  password_b: Option<String>,
  use_color: bool,
  keyfile_a: Option<&str>,
  keyfile_b: Option<&str>,
) {
  kdbx_to_sorted_vec(file_a, password_a, keyfile_a)
    .and_then(|a| kdbx_to_sorted_vec(file_b, password_b, keyfile_b).map(|b| (a, b)))
    .map(apply(&compare))
    .map(|r| {
      let mut i = 0;
      let mut stdout = StandardStream::stdout(ColorChoice::Always);
      while i < r.len() {
        match r.get(i) {
          Some(OnlyLeft(elem)) => {
            if use_color {
              stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                .unwrap();
            }
            println!("- {:?}", elem)
          }
          Some(OnlyRight(elem)) => {
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
    })
    .unwrap_or_else(|err| {
      println!("{}", err);
    });
}
