#!/usr/bin/env run-cargo-script
//! This is a regular crate doc comment, but it also contains a partial
//! Cargo manifest.  Note the use of a *fenced* code block, and the
//! `cargo` "language".
//!
//! ```cargo
//! [dependencies]
//! pulldown-cmark = "0.5.2"
//! ```

extern crate pulldown_cmark;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use pulldown_cmark::{Parser, Options, html};

fn main() -> std::io::Result<()> {
  let file = File::open("README.md")?;
  let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  buf_reader.read_to_string(&mut contents)?;
 
  // Set up options and parser. Strikethroughs are not part of the CommonMark standard
  // and we therefore must enable it explicitly.
  let mut options = Options::empty();
  options.insert(Options::ENABLE_STRIKETHROUGH); 
  let parser = Parser::new_ext(contents.as_str(), options);
 
  // Write to String buffer.
  let mut html_output = String::new();
  html::push_html(&mut html_output, parser);

  println!("{}", html_output);
  Ok(())
}
