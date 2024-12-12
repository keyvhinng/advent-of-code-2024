use std::fs::File;
use std::io::{BufRead, BufReader, Result};

const INPUT_FILE: &str = "input/in-example.txt";

fn main() -> Result<()> {
    let file = File::open(INPUT_FILE)?;
    let reader = BufReader::new(file);

    let mut stones: Vec<u64> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let stones_line: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        stones.extend(stones_line);
    }

    println!("stones = {:?}", stones);

    for _ in 0..25 {
        let mut new_stones = Vec::new();
        for stone in stones {
            if stone == 0 {
                new_stones.push(1);
            } else if stone.to_string().len() % 2 == 0 {
                let stone_str = stone.to_string();
                let half = stone_str.len() / 2;
                let left = stone_str[..half].parse::<u64>().unwrap();
                let right = stone_str[half..].parse::<u64>().unwrap();
                new_stones.push(left);
                new_stones.push(right);
            } else {
                new_stones.push(stone * 2024);
            }
        }
        stones = new_stones;
    }

    println!("Number of stones after 25 blinks: {}", stones.len());

    Ok(())
}
