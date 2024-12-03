use std::fs::File;
use std::io;
use std::io::BufRead;

const INPUT_FILE: &str = "input/in1.txt";

fn main()-> io::Result<()> {
    let input = File::open(INPUT_FILE)?;
    let buffered = io::BufReader::new(input);

    let mut ans: i32 = 0;

    for line in buffered.lines() {
        let mut safe: bool = true;
        let line: String = line?;
        let numbers: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();

        let n: usize = numbers.len();

        // check if numbers is either all increasing or decreasing
        let is_increasing: bool = numbers[0] < numbers[n-1];
        let is_decreasing: bool = numbers[0] > numbers[n-1];

        if !is_increasing && !is_decreasing {
            continue;
        }

        for i in 0..n-1 {
            if is_increasing {
                // println!("testing if increasing");
                if numbers[i] >= numbers[i + 1] {
                    safe = false;
                    break;
                }
            } else if is_decreasing {
                // println!("testing if decreasing");
                if numbers[i] <= numbers[i + 1] {
                    safe = false;
                    break;
                }
            }
        }

        // check if continuous pair of number are at most 3 apart
        for i in 0..n-1 {
            if (numbers[i] - numbers[i+1]).abs() > 3 {
                safe = false;
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
