use std::fs::File;
use std::io::{BufRead, BufReader, Result};

const INPUT_FILE: &str = "input/in.txt";

fn main() -> Result<()> {
    let input = File::open(INPUT_FILE)?;
    let buffered = BufReader::new(input);

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in buffered.lines() {
        let line = line.unwrap();
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    }

    let mut count: i32 = 0;

    let n_rows = matrix.len();
    let n_cols = matrix[0].len();

    for i in 0..n_rows {
        for j in 0..n_cols {
            if i + 2 < n_rows && j +2 < n_cols {
                if matrix[i][j] == 'M' && matrix[i+1][j+1]=='A' && matrix[i+2][j+2]=='S' && matrix[i][j+2]=='S' && matrix[i+2][j]=='M' {
                    count += 1;
                }
                if matrix[i][j] == 'M' && matrix[i+1][j+1]=='A' && matrix[i+2][j+2]=='S' && matrix[i][j+2]=='M' && matrix[i+2][j]=='S' {
                    count += 1;
                }
                if matrix[i][j] == 'S' && matrix[i+1][j+1]=='A' && matrix[i+2][j+2]=='M' && matrix[i][j+2]=='S' && matrix[i+2][j]=='M' {
                    count += 1;
                }
                if matrix[i][j] == 'S' && matrix[i+1][j+1]=='A' && matrix[i+2][j+2]=='M' && matrix[i][j+2]=='M' && matrix[i+2][j]=='S' {
                    count += 1;
                }
            }
        }
    }

    println!("Count: {}", count);

    Ok(())
}
