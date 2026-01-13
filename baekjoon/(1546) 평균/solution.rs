
use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: f64 = n.trim().parse().unwrap();

    let mut scores = String::new();
    io::stdin().read_line(&mut scores).unwrap();
    let scores: Vec<u8> = scores
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut sum: f64 = 0.0;
    scores
        .iter()
        .for_each(|&x| sum += (x as f64) / (*scores.iter().max().unwrap() as f64) * 100.0);

    println!("{}", sum / n);
}
