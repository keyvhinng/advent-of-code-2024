use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

const INPUT_FILE: &str = "input/in.txt";

fn dfs(
    i: isize,
    j: isize,
    ch: char,
    matrix: &Vec<Vec<char>>,
    nxt: i32,
    cc: &mut HashMap<(isize, isize), i32>,
) {
    cc.insert((i, j), nxt);

    let r = matrix.len();
    let c_len = matrix[0].len();

    let directions = vec![
        (-1, 0), // up
        (1, 0),  // down
        (0, -1), // left
        (0, 1),  // right
    ];

    for (dx, dy) in directions {
        let ni = i as isize + dx;
        let nj = j as isize + dy;

        if ni >= 0 && ni < r as isize && nj >= 0 && nj < c_len as isize {
            let ni = ni;
            let nj = nj;
            if matrix[ni as usize][nj as usize] == ch {
                if !cc.contains_key(&(ni, nj)) {
                    dfs(ni, nj, ch, matrix, nxt, cc);
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let file = File::open(INPUT_FILE)?;
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        matrix.push(line.chars().collect());
    }

    let mut ans: i32 = 0;
    let r = matrix.len();
    let c = matrix[0].len();

    let mut nxt = 0;

    let mut cc: HashMap<(isize, isize), i32> = HashMap::new();

    for i in 0..r {
        for j in 0..c {
            if !cc.contains_key(&(i as isize, j as isize)) {
                let ch = matrix[i][j];
                dfs(i as isize, j as isize, ch, &matrix, nxt, &mut cc);
                nxt += 1;
            }
        }
    }

    let mut ccs: HashMap<i32, Vec<(isize, isize)>> = HashMap::new();

    for (k, v) in cc.iter() {
        ccs.entry(*v).or_insert(Vec::new()).push(*k);
    }

    for (_, nodes) in ccs.iter() {
        let area = nodes.len() as i32;
        let mut per: Vec<((isize, isize), (isize, isize))> = Vec::new();

        for node in nodes {
            let directions = vec![
                (-1, 0), // up
                (1, 0),  // down
                (0, -1), // left
                (0, 1),  // right
            ];

            for (dx, dy) in directions {
                let ni = node.0 as isize + dx;
                let nj = node.1 as isize + dy;

                if ni < 0
                    || ni >= r as isize
                    || nj < 0
                    || nj >= c as isize
                    || !nodes.contains(&(ni, nj))
                {
                    per.push((*node, (ni, nj)));
                }
            }
        }

        let mut per_set: HashSet<((isize, isize), (isize, isize))> = HashSet::new();
        for p in per {
            per_set.insert(p);
        }
        let mut per_set_filtered: HashSet<((isize, isize), (isize, isize))> = HashSet::new();

        for (p1, p2) in per_set.iter() {
            let mut keep = true;
            for (dx, dy) in [(1, 0), (0, 1)] {
                let p1n = (p1.0 + dx, p1.1 + dy);
                let p2n = (p2.0 + dx, p2.1 + dy);

                if per_set.contains(&(p1n, p2n)) {
                    keep = false;
                }
            }

            if keep {
                per_set_filtered.insert((*p1, *p2));
            }
        }

        ans += area * per_set_filtered.len() as i32;
    }

    println!("{}", ans);

    Ok(())
}
