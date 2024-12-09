use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashSet;

const INPUT_FILE: &str = "input/in.txt";

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

            let dx = x1 as i32 - x2 as i32;
            let dy = y1 as i32 - y2 as i32;

            let mut next_x = x2 as i32 + dx;
            let mut next_y = y2 as i32 + dy;

            while next_x >= 0 && next_x < n_rows as i32 && next_y >= 0 && next_y < n_cols as i32 {
                antinodes.insert((next_x, next_y));

                next_x += dx;
                next_y += dy;
            }

            next_x = x1 as i32 - dx;
            next_y = y1 as i32 - dy;

            while next_x >= 0 && next_x < n_rows as i32 && next_y >= 0 && next_y < n_cols as i32 {
                antinodes.insert((next_x, next_y));

                next_x -= dx;
                next_y -= dy;
            }
        }
    }

    println!("Total unique antinodes: {}", antinodes.len());

    Ok(())
}
