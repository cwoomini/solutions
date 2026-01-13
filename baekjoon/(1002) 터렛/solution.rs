use std::io;

fn main() {
    let mut cnt = String::new();
    io::stdin().read_line(&mut cnt).expect("stdin");
    let cnt: i32 = cnt.trim().parse().expect("i32");

    for _ in 0..cnt {
        let mut inps = String::new();
        io::stdin().read_line(&mut inps).expect("stdin");
        let inps: Vec<i32> = inps
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("i32"))
            .collect();

        let d = (inps[3] - inps[0]).pow(2) + (inps[4] - inps[1]).pow(2);
        let sum = (inps[5] + inps[2]).pow(2);
        let dif = (inps[5] - inps[2]).pow(2);

        if d == 0 && inps[5] == inps[2] {
            println!("-1");
        } else if d == sum || d == dif {
            println!("1");
        } else if dif < d && d < sum {
            println!("2");
        } else {
            println!("0");
        }
    }
}