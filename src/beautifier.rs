use crate::Correction;
use difference::{Changeset, Difference};

pub const START_HTML1: &str = r##"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
        <title>"##;
pub const START_HTML2: &str = r##"
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
pub const END_HTML: &str = r#"
</body>
</html>
"#;

pub fn beautify_correction_for_msg(correc: Correction) {
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

fn beautify_for_msg(chngset: Changeset) {
    for (i, c) in chngset.diffs.iter().enumerate() {
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
            '.' | ',' | ' ' => print!("{}", c),
            _ => print!("{}\u{0338}", c),
        }
    }
}

pub fn beautify_correction_for_inline_html(correc: Correction) {
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

fn beautify_for_inline_html(chngset: Changeset) {
    for (i, c) in chngset.diffs.iter().enumerate() {
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
