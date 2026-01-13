use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());

    let mut n = String::new();
    reader.read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    let mut counting_arr: [usize; 10001] = [0; 10001];
    let mut input = String::new();

    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let input: usize = input.trim().parse().unwrap();
        counting_arr[input] += 1;
    }

    for i in 0..counting_arr.len() {
        for _ in 0..counting_arr[i] {
            writeln!(writer, "{}", i).unwrap();
        }
    }
}
