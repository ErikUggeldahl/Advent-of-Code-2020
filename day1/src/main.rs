use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let lines = lines
        .map(|x| x.expect("Line error").parse::<i32>().expect("Parse error"))
        .collect::<Vec<_>>();

    let (a, b) = lines
        .iter()
        .combinations(2)
        .find_map(|combo| match combo[..] {
            [a, b, ..] if a + b == 2020 => Some((a, b)),
            _ => None,
        })
        .expect("No 2020 pair");
    println!("{0} + {1} = {2}\n{0} * {1} = {3}", a, b, a + b, a * b);

    let (a, b, c) = lines
        .iter()
        .combinations(3)
        .find_map(|combo| match combo[..] {
            [a, b, c, ..] if a + b + c == 2020 => Some((a, b, c)),
            _ => None,
        })
        .expect("No 2020 pair");
    println!(
        "{0} + {1} + {2} = {3}\n{0} * {1} * {2} = {4}",
        a,
        b,
        c,
        a + b + c,
        a * b * c
    );

    Ok(())
}
