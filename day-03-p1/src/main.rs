use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE: &str = "input/in-example.txt";

fn main() -> io::Result<()> {
    let input = File::open(INPUT_FILE)?;
    let buffered = io::BufReader::new(input);

    let mut ans: i32 = 0;

    for line in buffered.lines() {
        let line = line?;
        let mut idx = 0;
        while idx < line.len() {
            if line[idx..].starts_with("mul(") {
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
                                ans += x * y;
                                idx += 4 + end_idx + 1;
                                continue;
                            }
                        }
                    }
                }
            }
            idx += 1;
        }
    }
    println!("ans: {}", ans);

    Ok(())
}
