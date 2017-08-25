extern crate adjacent_lines;

use std::env::args;
use adjacent_lines::visit;

fn eq(fst: &str, snd: &str, first: &mut bool, sep: &str) {
    if *first {
        *first = false;
        println!("{}", fst);
    }
    if fst == snd {
        println!("{}", sep);
    }
    println!("{}", snd);
}

fn main() {
    let input = std::io::stdin();
    let mut first = true;
    let separator = &args().nth(1).unwrap_or("EQUAL".into());
    visit(input.lock(), |a, b| eq(a, b, &mut first, separator));
}
