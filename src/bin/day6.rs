use std::collections::VecDeque;
use std::fs;
use std::ops::Range;

#[derive(Clone)]
enum Cell {
    Uninitialized,
    ClosestTo(usize, usize),
    MultipleClosest
}

fn get_bounds(points: &Vec<(usize, usize)>) -> (Range<usize>, Range<usize>) {
    let mut bounds = (std::usize::MAX..0, std::usize::MAX..0);
    for point in points.iter() {
        bounds.0.start = bounds.0.start.min(point.0);
        bounds.0.end = bounds.0.end.max(point.0);
        bounds.1.start = bounds.1.start.min(point.1);
        bounds.1.end = bounds.1.end.max(point.1);
    }
    bounds
}

fn manhattan_distance(lhs: &(usize, usize), rhs: &(usize, usize)) -> usize {
    ((lhs.0 as isize - rhs.0 as isize).abs() + (lhs.1 as isize - rhs.1 as isize).abs()) as usize
}

fn main() {
    let coords : Vec<(usize, usize)> = fs::read_to_string("resources/day6.input")
        .expect("Input file is missing.")
        .lines()
        .map(|l| {
            let mut parts = l.split(", ");
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap()
            )
        })
        .collect();

    let bounds = get_bounds(&coords);

    let width = bounds.0.end - bounds.0.start + 1;
    let height = bounds.1.end - bounds.1.start + 1;

    let mut queue = VecDeque::new();
    for (idx, coord) in coords.iter().enumerate() {
        queue.push_back((idx, 0, *coord));
    }

    let mut grid: Vec<Cell> = vec![Cell::Uninitialized; width * height];
    while let Some(cell) = queue.pop_front() {
        let coord = cell.2;
        let idx = (coord.1 - bounds.1.start) * width + (coord.0 - bounds.0.start);
        match grid[idx] {
            Cell::Uninitialized => {
                grid[idx] = Cell::ClosestTo(cell.0, cell.1);
            },
            Cell::ClosestTo(id, dist) => {
                if cell.1 == dist {
                    if id == cell.0 {
                        continue;
                    } else {
                        grid[idx] = Cell::MultipleClosest;
                    }
                } else if cell.1 < dist {
                    grid[idx] = Cell::ClosestTo(cell.0, cell.1);
                } else {
                    continue;
                }
            },
            Cell::MultipleClosest => continue
        }

        if coord.0 > bounds.0.start {
            queue.push_back((cell.0, cell.1 + 1, (coord.0 - 1, coord.1)));
        }

        if coord.0 < bounds.0.end {
            queue.push_back((cell.0, cell.1 + 1, (coord.0 + 1, coord.1)));
        }

        if coord.1 > bounds.1.start {
            queue.push_back((cell.0, cell.1 + 1, (coord.0, coord.1 - 1)));
        }

        if coord.1 < bounds.1.end {
            queue.push_back((cell.0, cell.1 + 1, (coord.0, coord.1 + 1)));
        }
    }

    let mut areas = vec![0; coords.len()];
    for cell in grid.iter() {
        if let Cell::ClosestTo(idx, _) = cell {
            areas[*idx] += 1;
        }
    }

    // Any point that has a cell on the boundary be closest to it, has an infinite area
    for x in 0..width {
        if let Cell::ClosestTo(idx, _) = grid[x] {
            areas[idx] = 0;
        }
    }

    for x in 0..width {
        if let Cell::ClosestTo(idx, _) = grid[(height - 1) * width + x] {
            areas[idx] = 0;
        }
    }

    for y in 0..height {
        if let Cell::ClosestTo(idx, _) = grid[y * width] {
            areas[idx] = 0;
        }
        if let Cell::ClosestTo(idx, _) = grid[y * width + width - 1] {
            areas[idx] = 0;
        }
    }

    let largest_area = areas
        .iter()
        .max_by(|a,b| a.cmp(b))
        .unwrap();

    println!("The largest area is of size {}.", largest_area);

    let mut count = 0;
    for y in bounds.1.start..=bounds.1.end {
        for x in bounds.0.start..=bounds.0.end {
            let total_distance: usize = coords
                .iter()
                .map(|coord| manhattan_distance(coord, &(x, y)))
                .sum();

            if total_distance < 10000 {
                count += 1;
            }
        }
    }

    println!("The size of the area that is within a total distance of 10,000 is {}.", count);
}
