use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE: &str = "input/in1.txt";

fn main() -> io::Result<()> {
    let input = File::open(INPUT_FILE)?;
    let buffered = io::BufReader::new(input);

    let mut ans: i32 = 0;

    let mut is_enabled = true;

    fn starts_with_at(s: &str, idx: usize, pat: &str) -> bool {
        s.get(idx..idx + pat.len()).map_or(false, |substr| substr == pat)
    }

    for line in buffered.lines() {
        let line = line?;
        let line_len = line.len();
        let mut idx = 0;
        while idx < line_len {
            if starts_with_at(&line, idx, "mul(") {
                if let Some(end_idx) = line[idx + 4..].find(')') {
                    let args = &line[idx + 4..idx + 4 + end_idx];
                    if let Some(comma_idx) = args.find(',') {
                        let x_str = args[..comma_idx].trim();
                        let y_str = args[comma_idx + 1..].trim();
                        if x_str.len() >= 1
                            && x_str.len() <= 3
                            && y_str.len() >= 1
                            && y_str.len() <= 3
                        {
                            if let (Ok(x), Ok(y)) = (x_str.parse::<i32>(), y_str.parse::<i32>()) {
                                if is_enabled {
                                    ans += x * y;
                                }
                                idx += 4 + end_idx + 1;
                                continue;
                            }
                        }
                    }
                }
                idx += 1;
            } else if starts_with_at(&line, idx, "do()") {
                is_enabled = true;
                idx += 4;
            } else if starts_with_at(&line, idx, "don't()") {
                is_enabled = false;
                idx += 7;
            } else {
                idx += 1;
            }
        }
    }
    println!("ans: {}", ans);

    Ok(())
}
