extern crate base64;
extern crate clap;
extern crate keepass;
extern crate rpassword;
extern crate termcolor;

pub mod diff;
pub mod stack;

use clap::Parser;
use diff::{group::Group, Diff, DiffDisplay};
use keepass::{error::DatabaseOpenError, Database, DatabaseKey};

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use std::fs::File;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Sets the first file
    #[clap(name = "INPUT-A", index = 1)]
    input_a: String,

    /// Sets the second file
    #[clap(name = "INPUT-B", index = 2)]
    input_b: String,

    /// Disables color output
    #[clap(short = 'C', long = "no-color")]
    no_color: bool,

    /// Enables verbose output
    #[clap(short = 'v', long)]
    verbose: bool,

    /// Enables verbose output
    #[clap(short = 'm', long = "mask-passwords")]
    mask_passwords: bool,

    /// Sets the password for the first file (will be asked for if omitted)
    #[clap(name = "password-a", long)]
    password_a: Option<String>,

    /// Sets the password for the second file (will be asked for if omitted)
    #[clap(name = "password-b", long)]
    password_b: Option<String>,

    /// Sets the password for both files (if it's the same for both files)
    #[clap(name = "passwords", long, short)]
    passwords: Option<String>,

    /// Asks for password only once, and tries to open both files with it
    #[clap(name = "same-password", long, short)]
    same_password: bool,

    /// Sets no password for the first file (and will not ask for it)
    #[clap(name = "no-password-a", long)]
    no_password_a: bool,

    /// Sets no password for the second file (and will not ask for it)
    #[clap(name = "no-password-b", long)]
    no_password_b: bool,

    /// Sets no password for both files (and will not ask for both files)
    #[clap(name = "no-passwords", long)]
    no_passwords: bool,

    /// Sets the key file for the first file
    #[clap(name = "keyfile-a", long)]
    keyfile_a: Option<String>,

    /// Sets the key file for the second file
    #[clap(name = "keyfile-b", long)]
    keyfile_b: Option<String>,

    /// Sets the same key file for both files (keyfile-a and keyfile-b would take precedence if set as well)
    #[clap(name = "keyfiles", long)]
    keyfiles: Option<String>,
}

fn main() -> Result<(), ()> {
    let arguments = Args::parse();

    match (arguments.input_a, arguments.input_b) {
        (file_a, file_b) => {
            let pass_a = match (
                arguments.password_a,
                arguments.passwords.clone(),
                arguments.same_password,
                arguments.no_password_a,
                arguments.no_passwords,
            ) {
                (Some(password), _, _, _, _) => Some(String::from(password)),
                (_, Some(password), _, _, _) => Some(String::from(password)),
                (_, _, true, _, _) => prompt_password("Password for both files: "),
                (_, _, _, true, _) => None,
                (_, _, _, _, true) => None,
                _ => prompt_password(format!("Password for file {}: ", file_a).as_str()),
            };
            let pass_b = match (
                arguments.password_b,
                arguments.passwords.clone(),
                arguments.same_password,
                arguments.no_password_b,
                arguments.no_passwords,
            ) {
                (Some(password), _, _, _, _) => Some(String::from(password)),
                (_, Some(password), _, _, _) => Some(String::from(password)),
                (_, _, true, _, _) => pass_a.clone(),
                (_, _, _, true, _) => None,
                (_, _, _, _, true) => None,
                _ => prompt_password(format!("Password for file {}: ", file_b).as_str()),
            };
            let keyfile_a: Option<String> = arguments.keyfile_a.or(arguments.keyfiles.clone());
            let keyfile_b: Option<String> = arguments.keyfile_b.or(arguments.keyfiles.clone());
            let use_color: bool = !arguments.no_color;
            let use_verbose: bool = arguments.verbose;
            let mask_passwords: bool = arguments.mask_passwords;

            let db_a = kdbx_to_group(file_a, pass_a, keyfile_a, use_verbose, mask_passwords)
                .expect("Error opening database A");
            let db_b = kdbx_to_group(file_b, pass_b, keyfile_b, use_verbose, mask_passwords)
                .expect("Error opening database B");

            let delta = db_a.diff(&db_b);

            println!(
                "{}",
                DiffDisplay {
                    inner: delta,
                    path: stack::Stack::empty(),
                    use_color,
                    use_verbose,
                    mask_passwords,
                }
            );
        }
    }

    Ok(())
}

fn prompt_password(prompt: &str) -> Option<String> {
    rpassword::prompt_password(prompt)
        .map(|s| if s == "" { None } else { Some(s) })
        .unwrap_or(None)
}

pub fn kdbx_to_group(
    file: String,
    password: Option<String>,
    keyfile_path: Option<String>,
    use_verbose: bool,
    mask_passwords: bool,
) -> Result<Group, DatabaseOpenError> {
    let db_key = get_database_key(password, keyfile_path)?;
    let db = Database::open(&mut File::open(file)?, db_key)?;
    Ok(Group::from_keepass(&db.root, use_verbose, mask_passwords))
}

fn get_database_key(
    password: Option<String>,
    keyfile_path: Option<String>,
) -> Result<DatabaseKey, std::io::Error> {
    let db_key = DatabaseKey::new();
    let db_key = match password {
        Some(pwd) => db_key.with_password(pwd.as_str()),
        _ => db_key,
    };
    if let Some(path) = keyfile_path {
        db_key.with_keyfile(&mut File::open(path)?)
    } else {
        Ok(db_key)
    }
}

pub fn set_fg(color: Option<Color>) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(color)).expect("Setting colors in your console failed. Please use the --no-color flag to disable colors if the error persists.");
}

pub fn reset_color() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.reset().expect("Resetting colors in your console failed. Please use the --no-color flag to disable colors if the error persists.");
}
