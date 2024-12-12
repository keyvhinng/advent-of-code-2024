use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

const INPUT_FILE: &str = "input/in.txt";

fn count(stone: u64, step: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if cache.contains_key(&(stone, step)) {
        return cache[&(stone, step)];
    }

    if step == 0 {
        return 1;
    }

    if stone == 0 {
        return count(1, step - 1, cache);
    }

    let stone_str = stone.to_string();
    let length = stone_str.len();

    if length % 2 == 0 {
        let first_half = stone_str[..length / 2].parse::<u64>().unwrap();
        let second_half = stone_str[length / 2..].parse::<u64>().unwrap();
        let result = count(first_half, step - 1, cache) + count(second_half, step - 1, cache);
        cache.insert((stone, step), result);
        return result;
    }

    let result = count(stone * 2024, step - 1, cache);
    cache.insert((stone, step), result);
    return result;
}

fn main() -> Result<()> {
    let file = File::open(INPUT_FILE)?;
    let reader = BufReader::new(file);
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();
    let mut stones: Vec<u64> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        stones.extend(line.split_whitespace().map(|s| s.parse::<u64>().unwrap()));
    }

    println!("stones = {:?}", stones);

    let mut ans = 0;

    for stone in stones.iter() {
        let result = count(*stone, 75, &mut cache);
        ans += result;
    }

    println!("ans: {}", ans);

    Ok(())
}
