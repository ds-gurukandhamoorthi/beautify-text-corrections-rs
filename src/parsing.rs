//NOM
use crate::Correction;
use nom::{
    bytes::complete::{is_not, tag},
    character::complete::char,
    combinator::opt,
    multi::{many0, separated_list},
    sequence::{delimited, tuple},
    IResult,
};

fn source_line(i: &str) -> IResult<&str, &str> {
    let minus = tuple((many0(char('\n')), char('-'), many0(char(' ')))); // "-[ ]*"
    let nl = char('\n');
    delimited(minus, is_not("\n"), nl)(i)
}

fn corrections_line(i: &str) -> IResult<&str, Vec<&str>> {
    let plus = tuple((char('+'), many0(char(' ')))); // "+[ ]*"
    let nl = char('\n');
    let (i, res) = delimited(plus, is_not("\n"), nl)(i)?;
    Ok((i, res.split("(OR)").collect::<Vec<&str>>()))
}

fn explanation_line(i: &str) -> IResult<&str, Option<&str>> {
    let expl_end = tag("]\n");
    opt(delimited(char('['), is_not("]"), expl_end))(i) //CAUTION: last coorection or explanation is expected to end with a new line. which is the normal use-case.
}

fn parse_a_correction(i: &str) -> IResult<&str, Correction> {
    let (i, src) = source_line(i).unwrap();
    let (i, correcs) = corrections_line(i).unwrap();
    let (i, expl) = explanation_line(i).unwrap();
    let correction = Correction {
        before: src,
        after: correcs,
        explanation: expl,
    };
    Ok((i, correction))
}

pub fn parse_all_corrections(i: &str) -> IResult<&str, Vec<Correction>> {
    let (i, c) = separated_list(char('\n'), parse_a_correction)(i).unwrap();
    Ok((i, c))
}

#[test]
fn parses_a_correction_preceded_by_whitespace(){
    let text = r##"

- bad text
+ correct text
"##;
    let result = parse_a_correction(text).ok().unwrap().1;
    let expected = Correction{before: "bad text", after: vec!["correct text"], explanation:None};
    assert_eq!(result.before, expected.before);
    assert_eq!(result.after, expected.after);
    assert_eq!(result.explanation, expected.explanation);
}

// NOTE: This may not be needed as files always ends with \n. Even echo "abc" > test; then test
// contains "abc\n"
// #[test]
// fn parses_a_correction_without_explanation_not_ending_with_newline(){
//     let text = r##"

// - bad text
// + correct text"##;
//     let result = parse_a_correction(text).ok().unwrap().1;
//     let expected = Correction{before: "bad text", after: vec!["correct text"], explanation:None};
//     assert_eq!(result.before, expected.before);
//     assert_eq!(result.after, expected.after);
//     assert_eq!(result.explanation, expected.explanation);
// }
