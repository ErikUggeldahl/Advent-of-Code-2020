use std::fs::File;
use std::io::{self, BufRead};

type Area = [Vec<char>];

fn trees_hit(area: &Area, width: usize, height: usize, run: usize, rise: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut tree_count = 0;

    while y < height - rise {
        x = (x + run) % width;
        y += rise;
        if area[y][x] == '#' {
            tree_count += 1
        }
    }

    tree_count
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;
    let area = io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("Read line error.").chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();
    let height = area.len();
    let width = area[0].len();

    let trees_hit_first = trees_hit(&area[..], width, height, 3, 1);

    println!("Trees hit on first: {}", trees_hit_first);

    let all_trees_hit = trees_hit(&area[..], width, height, 1, 1)
        * trees_hit(&area[..], width, height, 3, 1)
        * trees_hit(&area[..], width, height, 5, 1)
        * trees_hit(&area[..], width, height, 7, 1)
        * trees_hit(&area[..], width, height, 1, 2);

    println!("Product of all trees hit: {}", all_trees_hit);

    Ok(())
}
