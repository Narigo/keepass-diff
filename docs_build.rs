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
use pulldown_cmark::{Parser, Options, Event, Tag, CowStr, InlineStr, html};

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
      println!("this is header: {}", num);
      in_header_level = num;
      Event::Html(CowStr::Borrowed(""))
    },
    Event::Text(text) => {
      if in_header_level > 0 {
        println!("anchor {}", text);
        let result_html = format!("<h{}>{}</h{}>", in_header_level, text, in_header_level);
        println!("should convert to: {}", result_html);
        Event::Html(CowStr::Borrowed("<h1>this is me returning text"))
      } else {
        println!("not in header");
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

  println!("{}", html_output);
  Ok(())
}
