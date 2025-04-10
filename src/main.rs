use std::{env, io};
mod grep;

use grep::Grep;
fn main() {
    // let pattern = String::from(r"H\w\d[xyz]\d");
    // let mut scanner = Scanner::new(pattern);
    // let tokens: &Tokens = scanner.scan();
    // for token in tokens {
    //     println!("{:?}", token);
    // }

    // let mut matcher = Matcher::new(tokens);
    // matcher.match_tokens("Ha3xy1".to_owned());

    let mut buf = String::new();
    let pattern = env::args().nth(1).expect("failed getting the arg");

    io::stdin()
        .read_line(&mut buf)
        .expect("failed reading the input");

    let g = Grep::new(pattern, buf);
    g.match_pattern();
}
