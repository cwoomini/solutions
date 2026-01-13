use std::io::{self, BufRead, BufReader};

fn main() {
    let mut rd = BufReader::new(io::stdin().lock());

    let mut ln = String::new();
    rd.read_line(&mut ln).unwrap();
    let ln: Vec<&str> = ln.split_whitespace().collect();
    println!("{}", ln.len());
}
