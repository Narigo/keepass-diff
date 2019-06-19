#!/usr/bin/env run-cargo-script
//! This is a regular crate doc comment, but it also contains a partial
//! Cargo manifest.  Note the use of a *fenced* code block, and the
//! `cargo` "language".
//!
//! ```cargo
//! [dependencies]
//! heck = "0.3.1"
//! pulldown-cmark = "0.5.2"
//! ```

extern crate heck;
extern crate pulldown_cmark;

use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use pulldown_cmark::{Parser, Options, Event, Tag, CowStr, InlineStr, html};
use heck::KebabCase;

fn main() -> std::io::Result<()> {
  let file = File::open("README.md")?;
  let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  buf_reader.read_to_string(&mut contents)?;
 
  let options = Options::all();
  let parser = Parser::new_ext(contents.as_str(), options);
  let mut in_header_level = 0;
  let parser = parser.map(|event| match event {
    Event::Start(Tag::Header(num)) => {
      in_header_level = num;
      Event::Html(CowStr::Borrowed(""))
    },
    Event::Text(text) => {
      if in_header_level > 0 {
        let result_html = format!("<h{} id=\"{}\">{}</h{}>", in_header_level, text.to_kebab_case(), text, in_header_level);
        Event::Html(result_html.into())
      } else {
        Event::Text(text)
      }
    },
    Event::End(Tag::Header(_)) => {
      in_header_level = 0;
      event
    },
    _ => event
  });
 
  // Write to String buffer.
  let mut html_output = String::new();
  html::push_html(&mut html_output, parser);

  let template = File::open("docs_template.html")?;
  let mut contents = String::new();
  buf_reader = BufReader::new(template);
  buf_reader.read_to_string(&mut contents)?;
  contents = contents.replace("%%content%%", &html_output);

  let mut file = File::create("docs/index.html")?;
  file.write_all(contents.as_bytes())
}
