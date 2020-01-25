use difference::{Difference, Changeset};

use nom::{
    IResult,
    sequence::{delimited, preceded, tuple},
    character::complete::char,
    bytes::complete::{is_not, tag},
    combinator::opt,
    multi::{many0, separated_list},
    branch::alt,
};

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
    let text = "-  this is the old way\n+ this is the new way(OR)this is a newer way\n[explanation]\n\n";
    let (i, a) = source_line(text).unwrap();
    let (i, b) = corrections_line(i).unwrap();
    let b = b.split("(OR)").collect::<Vec<&str>>();
    println!("{}", i);
    println!("{}", a);
    println!("{:?}", b);
    let (i, c) = explanation_line(i).unwrap();
    let cor = Correction {
        before: a,
        after: &b,
        explanation: Some(c),
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
        Some(expl) => println!("_[{}]_", expl),
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

//NOM
fn source_line(i: &str) -> IResult<&str, &str>{
    let minus = tuple((char('-'), many0(char(' ')))); // "-[ ]*"
    let nl = char('\n');
    delimited(minus, is_not("\n"), nl)(i)
}


fn corrections_line(i: &str) -> IResult<&str, &str>{
    let plus = tuple((char('+'), many0(char(' ')))); // "+[ ]*"
    let nl = char('\n');
    delimited(plus, is_not("\n"), nl)(i)
}

fn explanation_line(i:&str) -> IResult<&str, &str>{
    delimited(char('['), is_not("]"), char(']'))(i) //CAUTION: last coorection or explanation may not end with a new line
}
