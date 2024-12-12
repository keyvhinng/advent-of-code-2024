use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

const INPUT_FILE: &str = "input/in.txt";

const DX: [i32; 4] = [-1, 0, 1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

fn dfs(
    x: usize,
    y: usize,
    height: i32,
    visited: &mut HashSet<(usize, usize)>,
    matrix: &Vec<Vec<i32>>,
    targets: &mut HashSet<(usize, usize)>,
) {
    let r = matrix.len();
    let c = matrix[0].len();

    if height == 9 {
        targets.insert((x, y));
        return;
    }

    visited.insert((x, y));

    for i in 0..4 {
        let nx = x as i32 + DX[i];
        let ny = y as i32 + DY[i];

        if nx < 0 || nx >= r as i32 || ny < 0 || ny >= c as i32 {
            continue;
        }

        let nx = nx as usize;
        let ny = ny as usize;

        if matrix[nx][ny] == height + 1 && !visited.contains(&(nx, ny)) {
            dfs(nx, ny, height + 1, visited, matrix, targets);
        }
    }

    visited.remove(&(x, y));
}

fn main() -> Result<()> {
    let file = File::open(INPUT_FILE)?;
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        matrix.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect(),
        );
    }

    let r = matrix.len();
    let c = matrix[0].len();
    let mut ans = 0;

    for i in 0..r {
        for j in 0..c {
            let c = matrix[i][j];
            if c == 0 {
                let mut visited = HashSet::new();
                let mut targets = HashSet::new();
                dfs(i, j, 0, &mut visited, &matrix, &mut targets);
                let score = targets.len() as i32;
                ans += score;
            }
        }
    }

    println!("{}", ans);

    Ok(())
}
