use std::io;

use line_reader;

const TREE: char = '#';

fn find_trees(input: &Vec<Vec<char>>, slope_x: usize, slope_y: usize) -> u32 {
    let mut counter = 0u32;
    for (step_number, line) in input.iter().step_by(slope_y).enumerate() {
        if line.get((step_number * slope_x) % line.len()).unwrap() == &TREE {
            counter += 1;
        }
    }

    counter
}

fn main() -> io::Result<()> {
    let lines: Vec<Vec<char>> = line_reader::convert("input")?
        .into_iter()
        .map(|f: String| f.chars().collect())
        .collect();

    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let product: u32 = slopes
        .iter()
        .map(|slope| find_trees(&lines, slope.0, slope.1))
        .product();

    println!("{}", product);
    Ok(())
}
