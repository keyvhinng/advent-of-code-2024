use std::fs::File;
use std::io::{self, BufRead, BufReader};

const INPUT_FILE: &str = "input/in1.txt";

fn main() -> io::Result<()> {
    let input = File::open(INPUT_FILE)?;
    let buffered = BufReader::new(input);

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in buffered.lines() {
        let line = line?;
        matrix.push(line.chars().collect());
    }

    let n_rows = matrix.len();
    let n_cols = matrix[0].len();

    /*
    We need to search word XMAS in the matrix. The word can be in any direction (up, down, left, right, diagonals).
     */

    let word: [char; 4] = ['X', 'M', 'A', 'S'];

    let mut total: i32 = 0;

    for i in 0..n_rows {
        for j in 0..n_cols {
            // check in the right direction
            if j + 3 < n_cols {
                let mut found = true;
                for k in 0..4 {
                    if matrix[i][j + k] != word[k] {
                        found = false;
                        break;
                    }
                }
                if found {
                    total += 1;
                }
            }

            // check in the down direction
            if i + 3 < n_rows {
                let mut found = true;
                for k in 0..4 {
                    if matrix[i + k][j] != word[k] {
                        found = false;
                        break;
                    }
                }
                if found {
                    total += 1;
                }
            }

            // check in the left direction
            if j >= 3 {
                let mut found = true;
                for k in 0..4 {
                    if matrix[i][j - k] != word[k] {
                        found = false;
                        break;
                    }
                }
                if found {
                    total += 1;
                }
            }

            // check in the up direction
            if i >= 3 {
                let mut found = true;
                for k in 0..4 {
                    if matrix[i - k][j] != word[k] {
                        found = false;
                        break;
                    }
                }
                if found {
                    total += 1;
                }
            }

            // check in the diagonal right-down direction
            if i + 3 < n_rows && j + 3 < n_cols {
                let mut found = true;
                for k in 0..4 {
                    if matrix[i + k][j + k] != word[k] {
                        found = false;
                        break;
                    }
                }
                if found {
                    total += 1;
                }
            }

            // check in the diagonal left-down direction
            if i + 3 < n_rows && j >= 3 {
                let mut found = true;
                for k in 0..4 {
                    if matrix[i + k][j - k] != word[k] {
                        found = false;
                        break;
                    }
                }
                if found {
                    total += 1;
                }
            }

            // check in the diagonal right-up direction
            if i >= 3 && j + 3 < n_cols {
                let mut found = true;
                for k in 0..4 {
                    if matrix[i - k][j + k] != word[k] {
                        found = false;
                        break;
                    }
                }
                if found {
                    total += 1;
                }
            }

            // check in the diagonal left-up direction
            if i >= 3 && j >= 3 {
                let mut found = true;
                for k in 0..4 {
                    if matrix[i - k][j - k] != word[k] {
                        found = false;
                        break;
                    }
                }
                if found {
                    total += 1;
                }
            }
        }
    }

    println!("total {}", total);

    Ok(())
}
