use std::fs::File;
use std::io::{self, BufRead, BufReader};

const INPUT_FILE: &str = "input/in-example.txt";

fn parse_updates(line: &str) -> Vec<u32> {
    let updates = line
        .split(",")
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    return updates;
}

fn main() -> io::Result<()> {
    let input = File::open(INPUT_FILE)?;
    let buffered = BufReader::new(input);

    let mut stage = 0;
    let mut rules = Vec::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    for line in buffered.lines() {
        let line = line?;
        if line.trim().is_empty() {
            stage = 1;
        } else if stage == 0 {
            let rule = line
                .split("|")
                .map(|x| x.trim().parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            rules.push(rule);
        } else {
            updates.push(parse_updates(&line));
        }
    }

    let mut ans: u32 = 0;

    // check if updates comply with the rules
    for update in &mut updates {
        let mut break_rule = false;
        for i in 0..update.len() {
            for j in i + 1..update.len() {
                for rule in &rules {
                    if rule[0] == update[j] && rule[1] == update[i] {
                        break_rule = true;
                        // swap the elements
                        let temp = update[i];
                        update[i] = update[j];
                        update[j] = temp;
                    }
                }
            }
        }
        if break_rule {
            let mid_element = update.len() / 2;
            ans += update[mid_element];
        }
    }

    println!("Answer: {}", ans);

    Ok(())
}
