use std::io::{self, BufRead, BufReader};

fn main() {
    let ln = BufReader::new(io::stdin().lock())
        .lines()
        .next()
        .unwrap()
        .unwrap();
    let mut ln = ln.trim().split_whitespace();
    let a: isize = ln.next().unwrap().trim().parse().unwrap();
    let b: isize = ln.next().unwrap().trim().parse().unwrap();

    println!("{}", a + b);
}