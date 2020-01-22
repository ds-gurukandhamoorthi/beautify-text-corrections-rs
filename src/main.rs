use difference::{Difference, Changeset};

#[derive(Debug)]
struct Correction<'a>{
    before: &'a str,
    after: &'a Vec<&'a str>,
    explanation: Option<&'a str>,
}

fn main() {
    let cor = Correction {
        before : "a new way",
        after : &vec!["another way", "one way"],
        explanation : Some("[this is how it works]"),
    };
    beautify_correction_for_msg(cor);
}

fn beautify_correction_for_msg(correc: Correction){
    for after_text in correc.after {
        let chngset = Changeset::new(correc.before, after_text, " ");
        beautify_for_msg(chngset);
        println!();
    }
    match correc.explanation {
        Some(expl) => println!("_{}_", expl),
            None => println!(),
        }
    println!();
}

fn beautify_for_msg(chngset: Changeset){
    for c in chngset.diffs {
        match c {
            Difference::Same(s) => print!("{}", s),
            Difference::Add(a) => print!("*{}*", a),
            Difference::Rem(r) => strike(&r),
        }
        print!(" ");
    }
}

fn strike(text: &str) {
    for c in text.chars() {
        match c {
            '.' => {print!("{}", c)},
            ',' => {print!("{}", c)},
            ' ' => {print!("{}", c)},
            _ => print!("{}\u{0338}", c),
        }
    }
}
