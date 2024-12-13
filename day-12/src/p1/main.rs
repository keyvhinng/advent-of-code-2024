use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

/*
--- Day 12: Garden Groups ---
Why not search for the Chief Historian near the gardener and his massive farm? There's plenty of food, so The Historians grab something to eat while they search.

You're about to settle near a complex arrangement of garden plots when some Elves ask if you can lend a hand. They'd like to set up fences around each region of garden plots, but they can't figure out how much fence they need to order or how much it will cost. They hand you a map (your puzzle input) of the garden plots.

Each garden plot grows only a single type of plant and is indicated by a single letter on your map. When multiple garden plots are growing the same type of plant and are touching (horizontally or vertically), they form a region. For example:

AAAA
BBCD
BBCC
EEEC
This 4x4 arrangement includes garden plots growing five different types of plants (labeled A, B, C, D, and E), each grouped into their own region.

In order to accurately calculate the cost of the fence around a single region, you need to know that region's area and perimeter.

The area of a region is simply the number of garden plots the region contains. The above map's type A, B, and C plants are each in a region of area 4. The type E plants are in a region of area 3; the type D plants are in a region of area 1.

Each garden plot is a square and so has four sides. The perimeter of a region is the number of sides of garden plots in the region that do not touch another garden plot in the same region. The type A and C plants are each in a region with perimeter 10. The type B and E plants are each in a region with perimeter 8.
The lone D plot forms its own region with perimeter 4.

Visually indicating the sides of plots in each region that contribute to the perimeter using - and |, the above map's regions' perimeters are measured as follows:

+-+-+-+
|A A A A|
+-+-+-+     +-+
                  |D|
+-+-+-+-+   +-+   +-+
|B B|   |C|
+   +   + +-+
|B B|   |C C|
+-+-+   +-+ +
          |C|
+-+-+-+   +-+
|E E E|
+-+-+-+
Plants of the same type can appear in multiple separate regions, and regions can even appear within other regions. For example:

OOOOO
OXOXO
OOOOO
OXOXO
OOOOO

The above map contains five regions, one containing all of the O garden plots, and the other four each containing a single X plot.

The four X regions each have area 1 and perimeter 4. The region containing 21 type O plants is more complicated; in addition to its outer edge contributing a perimeter of 20, its boundary with each X region contributes an additional 4 to its perimeter, for a total perimeter of 36.

Due to "modern" business practices, the price of fence required for a region is found by multiplying that region's area by its perimeter. The total price of fencing all regions on a map is found by adding together the price of fence for every region on the map.

In the first example, region A has price 4 * 10 = 40, region B has price 4 * 8 = 32, region C has price 4 * 10 = 40, region D has price 1 * 4 = 4, and region E has price 3 * 8 = 24. So, the total price for the first example is 140.

In the second example, the region with all of the O plants has price 21 * 36 = 756, and each of the four smaller X regions has price 1 * 4 = 4, for a total price of 772 (756 + 4 + 4 + 4 + 4).
*/

const INPUT_FILE: &str = "input/in-example.txt";

fn dfs(
    i: usize,
    j: usize,
    ch: char,
    visited: &mut HashSet<(usize, usize)>,
    matrix: &Vec<Vec<char>>,
    area: &mut i32,
    perimeter: &mut i32,
) {
    let r = matrix.len();
    let c_len = matrix[0].len();

    visited.insert((i, j));
    *area += 1;

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
            let ni = ni as usize;
            let nj = nj as usize;
            if matrix[ni][nj] == ch {
                if !visited.contains(&(ni, nj)) {
                    dfs(ni, nj, ch, visited, matrix, area, perimeter);
                }
            } else {
                *perimeter += 1;
            }
        } else {
            *perimeter += 1;
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

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..r {
        for j in 0..c {
            if !visited.contains(&(i, j)) {
                let ch = matrix[i][j];
                let mut area = 0;
                let mut perimeter = 0;
                dfs(i, j, ch, &mut visited, &matrix, &mut area, &mut perimeter);
                let price = area * perimeter;
                ans += price;
            }
        }
    }

    println!("{}", ans);

    Ok(())
}
