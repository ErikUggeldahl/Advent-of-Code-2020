use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Policy {
    character: char,
    min: u8,
    max: u8,
    password: String,
}

impl Policy {
    fn is_valid(&self) -> bool {
        let Policy {
            character,
            min,
            max,
            password,
        } = &self;
        let count = password.chars().filter(|c| *c == *character).count();
        count >= *min as usize && count <= *max as usize
    }

    fn is_valid2(&self) -> bool {
        let Policy {
            character,
            min,
            max,
            password,
        } = &self;
        let first = password
            .chars()
            .nth(*min as usize - 1)
            .expect("No first character.");
        let second = password
            .chars()
            .nth(*max as usize - 1)
            .expect("No second character.");

        (first == *character && second != *character)
            || (second == *character && first != *character)
    }
}

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("Line error."));

    let passwords = lines
        .map(|line| {
            let split = line
                .split_terminator(&['-', ' ', ':'][..])
                .collect::<Vec<_>>();
            Policy {
                character: split[2].parse().expect("Could not extract character."),
                min: split[0].parse().expect("Could not extract minimum"),
                max: split[1].parse().expect("Could not extract maximum"),
                password: String::from(split[4]),
            }
        })
        .collect::<Vec<_>>();

    let valid_count = passwords
        .iter()
        .filter(|password| password.is_valid())
        .count();
    println!("First star: {}", valid_count);

    let valid_count = passwords
        .iter()
        .filter(|password| password.is_valid2())
        .count();
    println!("Second star: {}", valid_count);

    Ok(())
}
