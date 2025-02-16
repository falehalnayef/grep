mod grep;

use std::{env, io};

use grep::Grep;
fn main() {

    let mut buf = String::new();
    let pattern = env::args().nth(1).expect("msg");


    io::stdin().read_line(&mut buf).expect("msg");


    let g = Grep::new(pattern, buf);

    match g.match_pattern() {
        Some(colored_string) => println!("{}", colored_string),
        None => println!("Pattern not found!"),
    }
}


