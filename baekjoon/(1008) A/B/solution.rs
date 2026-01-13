use std::io;

fn main() {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("str");
    let vals: Vec<i32> = inp
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("i32"))
        .collect();

    println!("{}", (vals[0] as f64) / (vals[1] as f64));
}
