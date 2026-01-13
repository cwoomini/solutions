use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: i16 = n.trim().parse().unwrap();

    for i in 1..=(2 * (n as i16) - 1) {
        let t = ((n - i) % n).abs() as usize;
        println!(
            "{:t$}{}",
            "",
            "*".repeat((1 + (n - (t as i16) - 1) * 2) as usize)
        );
    }
}
