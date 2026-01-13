use std::io;

fn main() {
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let mut x: u32 = x.trim().parse().unwrap();

    let mut i = 1;
    while x > i {
        x -= i;
        i += 1;
    }

    if i % 2 == 1 {
        println!("{}/{}", i - x + 1, 1 + x - 1);
    } else {
        println!("{}/{}", 1 + x - 1, i - x + 1);
    }
}
