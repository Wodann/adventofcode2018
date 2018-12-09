use std::collections::HashSet;
use std::fs;

fn main() {
    let input : Vec<i32> = fs::read_to_string("resources/day1.input")
        .expect("Input file is missing.")
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect();

    println!("Result frequency: {}", input.iter().fold(0, |a, b| a + b));

    let mut frequency = 0;
    let mut frequencies = HashSet::new();
    loop {
        for offset in input.iter() {
            frequency += offset;
            if !frequencies.insert(frequency) {
                println!("Reached frequency {} twice first!", frequency);
                return
            }
        }
    }
}