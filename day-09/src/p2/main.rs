use std::fs::File;
use std::io::{BufRead, BufReader, Result};

const INPUT_FILE: &str = "input/in.txt";

struct FileInfo {
    file_id: usize,
    start: usize,
    length: usize,
}

fn main() -> Result<()> {
    let file = File::open(INPUT_FILE)?;
    let reader = BufReader::new(file);

    let line = reader.lines().next().unwrap().unwrap();

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
    let mut files = Vec::new();

    while i < digits.len() {
        let length = digits[i];
        if is_file {
            // Add file blocks with the current file ID
            let start_pos = disk.len();
            for _ in 0..length {
                disk.push(Some(file_id));
            }
            let file_info = FileInfo {
                file_id,
                start: start_pos,
                length,
            };
            files.push(file_info);
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

    // Sort files in decreasing order of file ID
    files.sort_by(|a, b| b.file_id.cmp(&a.file_id));

    // Simulate moving files
    for file in &mut files {
        // Search for leftmost free space span to the left of the file
        let mut found = false;
        let mut span_start = 0;
        let mut pos = 0;

        while pos < file.start {
            // Find a free space block
            if disk[pos].is_none() {
                let span_start_candidate = pos;
                let mut span_length_candidate = 0;
                while pos < file.start && disk[pos].is_none() {
                    span_length_candidate += 1;
                    pos += 1;
                }
                if span_length_candidate >= file.length {
                    // Leftmost suitable span found
                    found = true;
                    span_start = span_start_candidate;
                    break;
                }
            } else {
                pos += 1;
            }
        }

        if found {
            // Move the file to the free space starting at span_start
            // Remove file blocks from original position
            for i in file.start..file.start + file.length {
                disk[i] = None;
            }
            // Place file blocks at new position
            for i in 0..file.length {
                disk[span_start + i] = Some(file.file_id);
            }
            // Update file start position
            file.start = span_start;
        }
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
