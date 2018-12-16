use std::collections::VecDeque;
use regex::Regex;
use std::fs;

struct Circle {
    queue: VecDeque<usize>
}

impl Circle {
    fn new(num_marbles: usize) -> Self {
        let mut queue = VecDeque::with_capacity(num_marbles);
        queue.push_back(0);
        Circle {
            queue
        }
    }

    fn insert(&mut self, value: usize) -> &mut Self {
        self.queue.push_front(value);
        self
    }

    fn remove(&mut self) -> usize {
        self.queue.pop_front().unwrap()
    }

    fn rotate_clockwise(&mut self, count: usize) -> &mut Self {
        for _ in 0..count {
            let value = self.queue.pop_front().unwrap();
            self.queue.push_back(value);
        }
        self
    }

    fn rotate_counter_clockwise(&mut self, count: usize) -> &mut Self {
        for _ in 0..count {
            let value = self.queue.pop_back().unwrap();
            self.queue.push_front(value);
        }
        self
    }
}

fn winning_score(last_marbles: usize, num_players: usize) -> usize {
    let mut scores = vec![0; num_players];
    let mut circle = Circle::new(last_marbles + 1);

    for turn in 1..=last_marbles {
        if turn % 23 == 0 {
            circle.rotate_counter_clockwise(7);
            scores[(turn - 1) % num_players] += turn + circle.remove();
        } else {
            circle.rotate_clockwise(2);
            circle.insert(turn);

        }
    }

    *scores.iter().max().unwrap()
}

fn main() {
    let input = fs::read_to_string("resources/day9.input")
        .expect("Input file is missing.");

    let reg = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    let (num_players, last_marble): (usize, usize) = {
        let cap = reg.captures(&input).unwrap();
        (cap[1].parse().unwrap(), cap[2].parse().unwrap())
    };

    println!("The winning elf's score is {}.", winning_score(last_marble, num_players));
    println!("The winning elf's score for 100x the number of marbles is {}.", winning_score(last_marble * 100, num_players));
}
