use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

const INPUT_FILE: &str = "input/in.txt";

const DX: [i32; 4] = [-1, 0, 1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

fn dfs(
    x: usize,
    y: usize,
    height: i32,
    matrix: &Vec<Vec<i32>>,
    memo: &mut HashMap<(usize, usize), i64>,
) -> i64 {
    let r = matrix.len();
    let c = matrix[0].len();

    if let Some(&res) = memo.get(&(x, y)) {
        return res;
    }

    if height == 9 {
        memo.insert((x, y), 1);
        return 1;
    }

    let mut total_paths = 0;

    for i in 0..4 {
        let nx = x as i32 + DX[i];
        let ny = y as i32 + DY[i];

        if nx < 0 || nx >= r as i32 || ny < 0 || ny >= c as i32 {
            continue;
        }

        let nx = nx as usize;
        let ny = ny as usize;

        if matrix[nx][ny] == height + 1 {
            total_paths += dfs(nx, ny, height + 1, matrix, memo);
        }
    }

    memo.insert((x, y), total_paths);
    total_paths
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
                let mut memo = HashMap::new();
                let rating = dfs(i, j, 0, &matrix, &mut memo);
                ans += rating;
            }
        }
    }

    println!("{}", ans);

    Ok(())
}
