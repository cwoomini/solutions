use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let mut n: u16 = n.trim().parse().unwrap();

    let mut v = 666;
    loop {
        if let Some(_) = v.to_string().find("666") {
            n -= 1;

            if n == 0 {
                break;
            }
        }
        v += 1;
    }

    println!("{}", v);
}
