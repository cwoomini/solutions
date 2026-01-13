use std::io::{self, BufRead, BufReader};

fn main() {
    let mut rd = BufReader::new(io::stdin().lock());

    let mut nm = String::new();
    rd.read_line(&mut nm).unwrap();
    let nm: Vec<usize> = nm
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut board: Vec<String> = Vec::new();
    for _ in 0..nm[0] {
        let mut ln = String::new();
        rd.read_line(&mut ln).unwrap();
        board.push(ln);
    }

    let mut min = 64;
    for n in 0..=(nm[0] - 8) {
        for m in 0..=(nm[1] - 8) {
            let diff = check_diff(&board, n, m);
            if diff < min {
                min = diff;
            }
        }
    }

    println!("{}", min);
}

fn check_diff(board: &Vec<String>, rn: usize, rm: usize) -> usize {
    let mut min = 64;

    for i in 0..2 {
        let mut total = 0;
        let mut tile = if i == 0 { 'W' } else { 'B' };

        for ln in &board[rn..(rn + 8)] {
            for ch in ln[rm..(rm + 8)].chars() {
                if ch != tile {
                    total += 1;
                    if total > min {
                        return min;
                    }
                }
                tile = if tile == 'W' { 'B' } else { 'W' };
            }
            tile = if tile == 'W' { 'B' } else { 'W' };
        }

        if total < min {
            min = total;
        }
    }
    min
}
