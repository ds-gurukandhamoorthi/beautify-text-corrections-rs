use difference::{Difference, Changeset};

fn main() {
    let changeset = Changeset::new("a new line", "another new line", " ");
    beautify_for_msg(changeset);
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
            '.' => {},
            ',' => {},
            ' ' => {},
            _ => print!("{}\u{0338}", c),
        }
    }
}
