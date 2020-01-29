use std::env;
use std::fs;

mod beautifier;
mod parsing;

#[derive(Debug)]
pub struct Correction<'a> {
    before: &'a str,
    after: Vec<&'a str>,
    explanation: Option<&'a str>,
}

fn main() {
    let mut args = env::args().skip(1);
    let filename = args.next().expect("Expected filename as first argument");
    let outformat = args.next();
    let contents = fs::read_to_string(&filename).expect("Cannot open file");
    let (_, cors) = parsing::parse_all_corrections(&contents).unwrap();
    let to_default_message_format = if let Some(fmt) = outformat {
        !(fmt == "html" || fmt == "htm")
    } else {
        true
    };
    beautifier::beautify_corrections(
        cors,
        if to_default_message_format {
            "msg"
        } else {
            "html"
        },
        &filename,
    );
}
