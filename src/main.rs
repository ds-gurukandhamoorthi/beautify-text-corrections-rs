use difference::{Difference, Changeset};


use nom::{
    IResult,
    sequence::{delimited, preceded, tuple},
    character::complete::{char, newline},
    bytes::complete::{is_not, tag, take_while, take},
    combinator::{opt, not},
    multi::{many0, many1,count, separated_list},
    branch::alt,
    many_till,
};

#[derive(Debug)]
struct Correction<'a>{
    before: &'a str,
    after: Vec<&'a str>,
    explanation: Option<&'a str>,
}

fn main() {
    let cor = Correction {
        before : "a new way",
        after : vec!["another way", "one way"],
        explanation : Some("[this is how it works]"),
    };
    beautify_correction_for_msg(cor);
    let text = "-  this is the old way\n+ this is the new way(OR)this is a newer way\n\n\n-bad expression\n+good exprsesion\n[words of explanation]\n";
    let (i, cors) = parse_all_corrections(text).unwrap();
    for cor in cors {
        beautify_correction_for_msg(cor);
    }
    println!(">{}<", i);
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
    let minus = tuple(( many0(char('\n')),char('-'), many0(char(' ')))); // "-[ ]*"
    let nl = char('\n');
    delimited(minus, is_not("\n"), nl)(i)
}

fn corrections_line(i: &str) -> IResult<&str, Vec<&str>>{
    let plus = tuple((char('+'), many0(char(' ')))); // "+[ ]*"
    let nl = char('\n');
    let (i, res) = delimited(plus, is_not("\n"), nl)(i)?;
    Ok((i, res.split("(OR)").collect::<Vec<&str>>()))
}

fn explanation_line(i:&str) -> IResult<&str, Option<&str>>{
    let expl_end = tuple((char(']'), many0(char('\n')))); // "-[ ]*"
    opt(delimited(char('['), is_not("]"), expl_end))(i) //CAUTION: last coorection or explanation may not end with a new line
}

fn parse_a_correction(i: &str) -> IResult<&str, Correction>{
    let (i, src) = source_line(i).unwrap();
    let (i, correcs) = corrections_line(i).unwrap();
    let (i, expl) = explanation_line(i).unwrap();
    let correction = Correction{
        before: src,
        after: correcs,
        explanation: expl,
    };
    Ok((i, correction))
}

fn parse_all_corrections(i: &str) -> IResult<&str, Vec<Correction>>{
    let (i, c) = separated_list(char('\n'), parse_a_correction)(i).unwrap();
    Ok((i, c))
}
