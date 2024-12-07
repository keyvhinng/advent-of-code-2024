use std::fs::File;
use std::io::{BufRead, BufReader, Result};

const INPUT_FILE: &str = "input/in.txt";

fn backtrack(operands: &Vec<i64>, result: i64, current: i64, index: usize) -> bool {
    if index == operands.len() {
        return current == result;
    }

    let mut next = current + operands[index];
    if backtrack(operands, result, next, index + 1) {
        return true;
    }

    next = current * operands[index];
    if backtrack(operands, result, next, index + 1) {
        return true;
    }

    next = format!("{}{}", current, operands[index]).parse().unwrap();
    if backtrack(operands, result, next, index + 1) {
        return true;
    }

    return false;
}



fn main() -> Result<()>  {
    let file = File::open(INPUT_FILE)?;
    let buffered = BufReader::new(file);

    let mut result: i64;
    let mut operands: Vec<i64>;
    let mut ans: i64 = 0;

    for line in buffered.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        let result_part = parts.next().unwrap();
        result = result_part[..result_part.len()-1].parse().unwrap();
        operands = parts.map(|x| x.parse().unwrap()).collect();

        if backtrack(&operands, result, operands[0], 1) {
            ans += result;
        }
    }

    println!("Answer: {}", ans);

    Ok(())
}
