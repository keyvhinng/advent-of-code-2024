use std::fs::File;
use std::io;
use std::io::BufRead;

const INPUT_FILE: &str = "input/in1.txt";

fn main()-> io::Result<()> {
    let input = File::open(INPUT_FILE)?;
    let buffered = io::BufReader::new(input);

    let mut ans: i32 = 0;

    for line in buffered.lines() {
        let mut safe: bool = false;
        let line: String = line?;
        let numbers: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();

        let n: usize = numbers.len();

        for ignore in 0..n {
            let mut is_decreasing: bool = true;
            let mut is_increasing: bool = true;
            let mut fulfill_condition: bool = true;

            for i in 0..n-1 {

                if i == ignore {
                    continue;
                }

                let mut next_number = numbers[i+1];

                if i+1 == ignore {
                    if i+2 == n {
                        continue;
                    }
                    next_number = numbers[i+2]
                }

                if numbers[i] <= next_number {
                    is_decreasing = false;
                }

                if numbers[i] >= next_number {
                    is_increasing = false;
                }

                if (numbers[i] - next_number).abs() > 3 {
                    fulfill_condition = false;
                }
            }

            if (is_increasing || is_decreasing) && fulfill_condition {
                safe = true;
                break;
            }
        }

        if safe == true {
            ans += 1;
        }
    }

    println!("ans: {}", ans);

    Ok(())
}
