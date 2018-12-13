extern crate regex;

use std::fs;

use regex::Regex;

#[derive(Debug)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize
}

const FABRIC_SIZE : usize = 1000;

fn is_unique(claim: &Claim, fabric: &Vec<u32>) -> bool {
    for y in claim.y..claim.y + claim.h {
        for x in claim.x..claim.x + claim.w {
            if fabric[y * FABRIC_SIZE + x] > 1 {
                return false
            }
        }
    }

    true
}

fn main() {
    let input = fs::read_to_string("resources/day3.input")
        .expect("Input file is missing.");

    let reg = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let claims : Vec<Claim> = input
        .lines()
        .map(|l| {
            let cap = reg.captures(l).unwrap();
            Claim {
                id: cap[1].parse().unwrap(),
                x: cap[2].parse().unwrap(),
                y: cap[3].parse().unwrap(),
                w: cap[4].parse().unwrap(),
                h: cap[5].parse().unwrap()
            }
        })
        .collect();

    let mut fabric = vec![0_u32; FABRIC_SIZE * FABRIC_SIZE];
    for claim in claims.iter() {
        for y in claim.y..claim.y + claim.h {
            for x in claim.x..claim.x + claim.w {
                fabric[y * FABRIC_SIZE + x] += 1;
            }
        }
    }

    println!("{} square inches are within two or more claims.", fabric.iter().filter(|c| **c > 1).count());

    for claim in claims.iter() {
        if is_unique(&claim, &fabric) {
            println!("Claim #{} is unique.", claim.id);
            break;
        }
    }
}