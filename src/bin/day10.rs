use regex::Regex;
use std::fs;

#[derive(Clone, Copy, Debug)]
struct Bounds {
    min: (i32,i32),
    max: (i32,i32)
}

impl Bounds {
    fn shrunk(&self, other: &Bounds) -> bool {
        self.min.0 >= other.min.0
            && self.min.1 >= other.min.1
            && self.max.0 <= other.max.0
            && self.max.1 <= other.max.1
    }
}

fn translate(positions: &Vec<(i32,i32)>, velocities: &Vec<(i32,i32)>) -> Vec<(i32,i32)> {
    positions
        .iter()
        .zip(velocities)
        .map(|((x,y),(vx,vy))| (x+vx, y+vy))
        .collect()
}

fn get_bounds(positions: &Vec<(i32,i32)>) -> Bounds {
    Bounds {
        min: positions.iter().fold((std::i32::MAX ,std::i32::MAX), |(x1,y1),(x2,y2)| (x1.min(*x2), y1.min(*y2)) ),
        max: positions.iter().fold((std::i32::MIN, std::i32::MIN), |(x1,y1),(x2,y2)| (x1.max(*x2), y1.max(*y2)) )
    }
}

fn display(positions: &Vec<(i32, i32)>, bounds: &Bounds) {
    let width = (bounds.max.0 - bounds.min.0 + 1) as usize;
    let height = (bounds.max.1 - bounds.min.0 + 1) as usize;

    let mut grid = vec![false; width * height];
    for pos in positions.iter() {
        let x = (pos.0 - bounds.min.0) as usize;
        let y = (pos.1 - bounds.min.1) as usize;

        grid[y * width + x] = true;
    }

    for y in 0..height {
        for x in 0..width {
            print!("{}", if grid[y * width + x] {
                "#"
            } else {
                "."
            });
        }
        println!();
    }
}

fn main() {
    let reg = Regex::new(r"position=<\s*([\d-]+),\s*([\d\s-]+)> velocity=<\s*([\d\s-]+),\s*([\d-]+)>").unwrap();

    let (mut new_positions, velocities): (Vec<(i32,i32)>, Vec<(i32,i32)>) = fs::read_to_string("resources/day10.input")
        .expect("Input file is missing.")
        .lines()
        .map(|l| {
            let cap = reg.captures(&l).unwrap();
            (
                (cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                (cap[3].parse().unwrap(), cap[4].parse().unwrap())
            )
        })
        .unzip();

    let mut prev_bounds = Bounds { min: (std::i32::MIN, std::i32::MIN), max: (std::i32::MAX, std::i32::MAX) };
    let mut bounds = get_bounds(&new_positions);

    let mut positions = Vec::new();
    let mut time = 0;
    while bounds.shrunk(&prev_bounds) {
        positions = new_positions;
        new_positions = translate(&positions, &velocities);
        time += 1;

        prev_bounds = bounds;
        bounds = get_bounds(&new_positions);
    }

    display(&positions, &bounds);
    println!("It took {} seconds to reach this result.", time - 1);
}
