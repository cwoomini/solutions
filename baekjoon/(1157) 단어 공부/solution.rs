use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader},
};

fn main() {
    let mut rd = BufReader::new(io::stdin().lock());

    let mut word = String::new();
    rd.read_line(&mut word).unwrap();
    let word = word.trim().to_uppercase();

    let mut letters = HashMap::new();
    for lt in word.chars() {
        *letters.entry(lt).or_insert(1) += 1;
    }

    let max = letters.values().max().unwrap();
    let cnt: Vec<char> = letters
        .iter()
        .filter(|&(_, c)| c == max)
        .map(|(&ch, _)| ch)
        .collect();

    if cnt.len() == 1 {
        println!("{}", cnt[0]);
    } else {
        println!("?");
    }
}
