use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    let mut layer = 1;
    let mut layer_max = 1;
    let mut max_add = 6;

    while n > layer_max {
        layer_max += max_add;
        max_add += 6;
        layer += 1;
    }

    println!("{}", layer);
}
