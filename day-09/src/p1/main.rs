use std::fs::File;
use std::io::{BufRead, BufReader, Result};

const INPUT_FILE: &str = "input/in.txt";

fn main() -> Result<()> {
    let file = File::open(INPUT_FILE)?;
    let reader = BufReader::new(file);

    let line = reader.lines().next().unwrap().unwrap();

    println!("line: {}", line);

    // Parse the input line into a list of segment lengths
    let digits: Vec<usize> = line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    // Create the initial disk layout
    let mut disk: Vec<Option<usize>> = Vec::new();
    let mut is_file = true;
    let mut file_id = 0;
    let mut i = 0;

    while i < digits.len() {
        let length = digits[i];
        if is_file {
            // Add file blocks with the current file ID
            for _ in 0..length {
                disk.push(Some(file_id));
            }
            file_id += 1;
        } else {
            // Add free space blocks
            for _ in 0..length {
                disk.push(None);
            }
        }
        is_file = !is_file;
        i += 1;
    }

    // Simulate moving file blocks to the leftmost free space
    loop {
        // Find the leftmost free space
        let leftmost_free = disk.iter().position(|&block| block.is_none());
        if leftmost_free.is_none() {
            break;
        }
        let leftmost_free = leftmost_free.unwrap();

        // Find the last file block
        let last_file = disk.iter().rposition(|&block| block.is_some());
        if last_file.is_none() {
            break;
        }
        let last_file = last_file.unwrap();
        if last_file <= leftmost_free {
            break;
        }

        // Move the last file block to the leftmost free space
        disk[leftmost_free] = disk[last_file];
        disk[last_file] = None;
    }


    // Calculate the checksum
    let checksum: usize = disk
        .iter()
        .enumerate()
        .filter_map(|(pos, block)| block.map(|id| pos * id))
        .sum();

    println!("Checksum: {}", checksum);

    Ok(())
}
