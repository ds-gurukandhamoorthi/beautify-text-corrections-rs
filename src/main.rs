use std::fs;
use std::env;
use difference::{Difference, Changeset};

mod parsing;

#[derive(Debug)]
pub struct Correction<'a>{
    before: &'a str,
    after: Vec<&'a str>,
    explanation: Option<&'a str>,
}

const START_HTML1 : &str= r##"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
        <title>"##;
const START_HTML2 : &str = r##"
        </title>
            <style>
                del, ins {
                        text-decoration: none;
                            }
    del {
            background-color: #fbb6c2;
                }
    ins {
            background-color: #d4fcbc;
                }
    em {
            color: gray;
                       }
    </style>
    </head>
    <body>
"##;
const END_HTML: &str = r#"
</body>
</html>
"#;


fn main() {
    let mut args = env::args().skip(1);
    let filename = args.next().expect("Expected filename as first argument");
    let outformat = args.next();
    let contents = fs::read_to_string(&filename).expect("Cannot open file");
    let (_, cors) = parsing::parsing::parse_all_corrections(&contents).unwrap();
    let to_default_message_format =
        if let Some(fmt) = outformat {
            !(fmt == "html" || fmt == "htm")
        }else {
            true
        };
    if to_default_message_format{
        for cor in cors {
            beautify_correction_for_msg(cor);
        }
    } else {
        println!("{}", START_HTML1);
        println!("{}", &filename);
        println!("{}", START_HTML2);
        for cor in cors {
            beautify_correction_for_inline_html(cor);
        }
        println!("{}", END_HTML);
    }
}

fn beautify_correction_for_msg(correc: Correction){
    for after_text in correc.after {
        let chngset = Changeset::new(correc.before, after_text, " ");
        beautify_for_msg(chngset);
        println!();
    }
    match correc.explanation {
        Some(expl) => println!("_[{}]_", expl),
            None => println!(),
        }
}

fn beautify_for_msg(chngset: Changeset){
    for (i,c) in chngset.diffs.iter().enumerate() {
        if i != 0 {
            print!(" ");
        }
        match c {
            Difference::Same(s) => print!("{}", s),
            Difference::Add(a) => print!("*{}*", a),
            Difference::Rem(r) => strike(&r),
        }
    }
}

fn strike(text: &str) {
    for c in text.chars() {
        match c {
            '.' | ',' | ' ' => {print!("{}", c)},
            _ => print!("{}\u{0338}", c),
        }
    }
}

fn beautify_correction_for_inline_html(correc: Correction){
    for after_text in correc.after {
        let chngset = Changeset::new(correc.before, after_text, " ");
        beautify_for_inline_html(chngset);
        print!("<br>");
    }
    match correc.explanation {
        Some(expl) => println!("<em>[{}]</em>", expl),
            None => print!("<br>"),
        }
}


fn beautify_for_inline_html(chngset: Changeset){
    for (i,c) in chngset.diffs.iter().enumerate() {
        if i != 0 {
            print!(" ");
        }
        match c {
            Difference::Same(s) => print!("{}", s),
            Difference::Add(a) => print!("<ins>{}</ins>", a),
            Difference::Rem(r) => print!("<del>{}</del>", r),
        }
    }
}
