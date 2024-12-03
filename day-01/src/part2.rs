use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE: &str = "input/in2.txt";

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

    let result: i32 = a
        .iter()
        .map(|number| {
            number * b
                .iter()
                .filter(|r| &number == r)
                .count() as i32
    }).sum();



    println!("the result is: {}", result);

    Ok(())
}
