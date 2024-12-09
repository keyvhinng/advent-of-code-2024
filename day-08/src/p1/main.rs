use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashSet;

const INPUT_FILE: &str = "input/in-example.txt";

fn main() -> Result<()> {
    let file = File::open(INPUT_FILE)?;
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        matrix.push(line.chars().collect());
    }

    let n_rows = matrix.len();
    let n_cols = matrix[0].len();

    let mut antennas: Vec<(usize, usize, char)> = Vec::new();

    for i in 0..n_rows {
        for j in 0..n_cols {
            let c = matrix[i][j];
            if c != '.' {
                antennas.push((i, j, c));
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            let (x1, y1, c1) = antennas[i];
            let (x2, y2, c2) = antennas[j];

            if c1 != c2 {
                continue;
            }

            // First antinode
            let ax1 = 2 * x1 as i32 - x2 as i32;
            let ay1 = 2 * y1 as i32 - y2 as i32;

            if ax1 >= 0 && ax1 < n_rows as i32 && ay1 >= 0 && ay1 < n_cols as i32 {
                antinodes.insert((ax1, ay1));
            }

            // Second antinode
            let ax2 = 2 * x2 as i32 - x1 as i32;
            let ay2 = 2 * y2 as i32 - y1 as i32;

            if ax2 >= 0 && ax2 < n_rows as i32 && ay2 >= 0 && ay2 < n_cols as i32 {
                antinodes.insert((ax2, ay2));
            }
        }
    }

    println!("Total unique antinodes: {}", antinodes.len());

    Ok(())
}
