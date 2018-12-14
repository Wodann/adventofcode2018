use std::fs;

fn react_polymer(polymer: std::str::Chars) -> Vec<char> {
    let mut collapsed: Vec<char> = Vec::new();

    for curr in polymer {
        if let Some(prev) = collapsed.last() {
            if prev.is_uppercase() && curr.is_lowercase() {
                if curr.to_ascii_uppercase() == *prev {
                    collapsed.remove(collapsed.len() - 1);
                    continue;
                }
            } else if prev.is_lowercase() && curr.is_uppercase() {
                if curr.to_ascii_lowercase() == *prev {
                    collapsed.remove(collapsed.len() - 1);
                    continue;
                }
            }
        }
        collapsed.push(curr);
    }
    collapsed
}

fn main() {
    let input = fs::read_to_string("resources/day5.input")
        .expect("Input file is missing.");

    let polymer = react_polymer(input.chars());

    println!("The resulting polymer is {} units long.", polymer.len());

    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let shortest_polymer = alphabet
        .chars()
        .map(|c| {
            react_polymer(input
                .chars()
                .filter(|unit| *unit != c && *unit != c.to_ascii_lowercase())
                .collect::<String>()
                .chars()
            )
        })
        .min_by(|a, b| a.len().cmp(&b.len()))
        .unwrap();

    println!("The shortest polymer that can be produced by removing an element is {} units long.", shortest_polymer.len());
}