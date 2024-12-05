use std::fs::File;
use std::io::{self, BufRead, BufReader};

/*
Problem statement:
The notation X|Y means that if both page number X and page number Y are to be produced as part of an update,
page number X must be printed at some point before page number Y.

For example:

47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47

The first section specifies the page ordering rules, one per line. The first rule, 47|53,
means that if an update includes both page number 47 and page number 53, then page number 47 must be printed at some point before page number 53.
(47 doesn't necessarily need to be immediately before 53; other pages are allowed to be between them.)
The second section specifies the page numbers of each update. Because most safety manuals are different,
the pages needed in the updates are different too. The first update, 75,47,61,53,29,
means that the update consists of page numbers 75, 47, 61, 53, and 29.
 */

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
