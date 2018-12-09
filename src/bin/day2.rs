use std::fs;

fn main() {
    let input = fs::read_to_string("resources/day2.input")
        .expect("Input file is missing.");

    let lines : Vec<&[u8]> = input
        .lines()
        .map(|l| l.as_bytes())
        .collect();

    let counts : Vec<[usize; 26]> = lines
        .iter()
        .map(|l| {
            let mut counts = [0; 26];
            for c in l.iter() {
                counts[(c - 97) as usize] += 1;
            }
            counts
        })
        .collect();

    let twos_count = counts
        .iter()
        .filter(|counts| counts.iter().any(|c| *c == 2))
        .count();

    let threes_count = counts
        .iter()
        .filter(|counts| counts.iter().any(|c| *c == 3))
        .count();

    println!("Checksum: {}", twos_count * threes_count);

    for (idx, lhs) in lines.iter().enumerate() {
        for rhs in lines[idx+1..].iter() {
            if lhs.iter().zip(rhs.iter()).filter(|(lhs, rhs)| lhs != rhs).count() == 1 {
                let same_chars : Vec<u8> = lhs.iter().zip(rhs.iter())
                    .filter_map(|(lhs, rhs)| if lhs == rhs { Some(*lhs) } else { None })
                    .collect();

                println!("Part of ID that is the same: {}", String::from_utf8(same_chars).unwrap());
                return;
            }
        }
    }
}