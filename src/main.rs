extern crate pulldown_cmark;
extern crate structopt;

use pulldown_cmark::{html, Parser};
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "rust_wasi_markdown_parser",
    about = "Markdown to HTML renderer CLI, written with Rust & WASI"
)]
pub struct Options {
    #[structopt(parse(from_os_str))]
    filename: PathBuf,
}

fn main() {
    let options = Options::from_args();

    let contents = fs::read_to_string(options.filename)
        .unwrap_or_else(|e| panic!("fs::read_to_string() failed: {e}"));

    let result = render_markdown(contents);

    println!("{}", result);
}

pub fn render_markdown(markdown: String) -> String {
    let mut html_buf = String::new();
    let parser = Parser::new(&markdown[..]);
    html::push_html(&mut html_buf, parser);
    html_buf
}
