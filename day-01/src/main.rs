use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE: &str = "input/in.txt";

fn main() -> io::Result<()> {

    let input = File::open(INPUT_FILE)?;
    let buffered = io::BufReader::new(input);

    let mut a = Vec::new();
    let mut b = Vec::new();

    for line in buffered.lines() {
        let line = line?;
        let numbers: Vec<i32> = line.split_whitespace()
                                    .map(|s| s.parse().unwrap())
                                    .collect();
        a.push(numbers[0]);
        b.push(numbers[1]);
    }

    a.sort();
    b.sort();

    let sum_abs_diff: i32 = a.iter().zip(b.iter())
                                .map(|(x, y)| (x - y).abs())
                                .sum();
    println!("Sum of absolute differences: {}", sum_abs_diff);

    Ok(())
}
