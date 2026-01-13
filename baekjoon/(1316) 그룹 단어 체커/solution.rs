use std::{collections::HashSet, io};

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: u32 = n.trim().parse().unwrap();

    let mut res = 0;

    'outer: for _ in 0..n {
        let mut word = String::new();
        io::stdin().read_line(&mut word).unwrap();

        let mut set = HashSet::new();
        let mut prev: char = word.chars().next().unwrap();

        for letter in word.chars() {
            if prev != letter && set.contains(&letter) {
                continue 'outer;
            }
            set.insert(letter);
            prev = letter;
        }
        res += 1;
    }

    println!("{}", res);
}
