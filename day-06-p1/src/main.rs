use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;

const INPUT_FILE: &str = "input/in.txt";

fn main() -> io::Result<()> {
    let input = File::open(INPUT_FILE)?;
    let buffered = BufReader::new(input);

    let mut mat: Vec<Vec<char>> = Vec::new();
    let mut initial_pos = (0, 0);

    for (i, line) in buffered.lines().enumerate() {
        let line = line?;
        let mut chars: Vec<char> = line.chars().collect();
        if let Some(y) = chars.iter().position(|&c| c == '^') {
            initial_pos = (i as i32, y as i32);
            chars[y] = '.'; // Replace guard symbol with empty space
        }
        mat.push(chars);
    }

    // simulate
    let mut pos: (i32, i32) = initial_pos;
    let mut dir = 0; // 0: up, 1: right, 2: down, 3: left
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(pos);

    loop {
        let (x, y) = pos;
        let mut next_pos: (i32, i32) = pos;
        let mut turned = false;

        match dir {
            0 => { // Up
                if x > 0 && mat[(x - 1) as usize][y as usize] == '#' {
                    dir = 1;
                    turned = true;
                } else {
                    next_pos = (x - 1, y);
                }
            }
            1 => { // Right
                if y < mat[x as usize].len() as i32 - 1 && mat[x as usize][(y + 1) as usize] == '#' {
                    dir = 2;
                    turned = true;
                } else {
                    next_pos = (x, y + 1);
                }
            }
            2 => { // Down
                if x < mat.len() as i32 - 1 && mat[(x + 1) as usize][y as usize] == '#' {
                    dir = 3;
                    turned = true;
                } else {
                    next_pos = (x + 1, y);
                }
            }
            3 => { // Left
                if y > 0 && mat[x as usize][(y - 1) as usize] == '#' {
                    dir = 0;
                    turned = true;
                } else {
                    next_pos = (x, y - 1);
                }
            }
            _ => panic!("Invalid direction"),
        }

        if !turned {
            pos = next_pos;

            if pos.0 < 0 || pos.0 >= mat.len() as i32 || pos.1 < 0 || pos.1 >= mat[0].len() as i32 {
                break;
            }
            visited.insert(pos);
        }
    }

    println!("Distinct positions visited: {}", visited.len());

    Ok(())
}
