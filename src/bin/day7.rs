use regex::Regex;
use std::fs;

struct State {
    order: Vec<u8>,
    finished: [bool; 26],
    queue: [bool; 26],
    workers: [(u32,Option<u8>); 5],
    time: u32
}

impl State {
    fn default() -> State {
        State {
            order: Vec::new(),
            finished: [false; 26],
            queue: [false; 26],
            workers: [(0, None); 5],
            time: 0
        }
    }

    fn next_step(&mut self) -> Option<u8> {
        for step in 0..26 {
            if self.queue[step] {
                self.finished[step] = true;
                self.order.push(step as u8 + 65);
                self.queue[step] = false;
                return Some(step as u8);
            }
        }
        None
    }

    fn has_work(&self) -> bool {
        if self.queue.iter().filter(|w| **w).count() > 0 {
            return true
        }

        if self.workers.iter().filter(|(_,w)| w.is_some()).count() > 0 {
            return true
        }

        false
    }

    fn join_step(&mut self) -> Option<Vec<u8>> {
        let mut busy_workers: Vec<&mut (u32, Option<u8>)> = self.workers
            .iter_mut()
            .filter(|(_,w)| w.is_some())
            .collect();

        if busy_workers.iter().count() > 0 {
            let shortest_time = busy_workers
                .iter()
                .min_by(|(t1,_),(t2,_)| t1.cmp(t2))
                .map(|(t,_)| *t)
                .unwrap();

            self.time = shortest_time;

            let mut finished = Vec::new();
            for (_,step) in busy_workers.iter_mut().filter(|(t,_)| *t == shortest_time) {
                let step = step.take().unwrap();
                self.finished[step as usize] = true;
                self.order.push(step + 65);

                finished.push(step);
            }
            return Some(finished)
        }

        None
    }

    fn spawn_step(&mut self) -> bool {
        let free_worker = self.workers
            .iter_mut()
            .filter(|(_,w)| w.is_none())
            .min_by(|(t1,_),(t2,_)| t1.cmp(t2));

        if let None = free_worker {
            return false;
        }

        let mut free_worker = free_worker.unwrap();

        for step in 0..26 {
            if self.queue[step] {
                free_worker.0 = self.time + 61_u32 + step as u32;
                free_worker.1.replace(step as u8);
                self.queue[step] = false;
                return true
            }
        }

        false
    }

    fn unlock_step(&mut self, step: u8) ->&mut Self {
        self.queue[step as usize] = true;
        self
    }
}

fn main() {
    let input = fs::read_to_string("resources/day7.input")
        .expect("Input file is missing.");

    let reg = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();

    let dependencies: Vec<(u8,u8)> = input
        .lines()
        .map(|l| {
            let cap = reg.captures(l).unwrap();
            (cap[1].as_bytes()[0] - 65, cap[2].as_bytes()[0] - 65)
        })
        .collect();

    let mut state = State::default();

    let mut assumptions = [false; 26];
    for (a, _) in dependencies.iter() {
        assumptions[*a as usize] = true;
    }
    for (_, b) in dependencies.iter() {
        assumptions[*b as usize] = false;
    }

    for (step,_) in assumptions.iter().enumerate().filter(|(_,b)| **b) {
        state.unlock_step(step as u8);
    }

    while let Some(step) = state.next_step() {
        for (_,dep) in dependencies.iter().filter(|(a,_)| *a == step) {     // Dependencies
            let mut unlock = true;
            for (req,_) in dependencies.iter().filter(|(_,d)| *d == *dep) { // Requirements
                if !state.finished[*req as usize] {
                    unlock = false;
                    break;
                }
            }
            if unlock {
                state.unlock_step(*dep);
            }
        }
    }

    println!("The order of operation is: {}.", String::from_utf8(state.order).unwrap());

    state = State::default();
    for (step,_) in assumptions.iter().enumerate().filter(|(_,b)| **b) {
        state.unlock_step(step as u8);
    }

    while state.has_work() {
        while state.spawn_step() {}

        if let Some(steps) = state.join_step() {
            for step in steps.iter() {
                for (_, dep) in dependencies.iter().filter(|(a, _)| *a == *step) {     // Dependencies
                    let mut unlock = true;
                    for (req, _) in dependencies.iter().filter(|(_, d)| *d == *dep) { // Requirements
                        if !state.finished[*req as usize] {
                            unlock = false;
                            break;
                        }
                    }
                    if unlock {
                        state.unlock_step(*dep);
                    }
                }
            }
        }
    }

    println!("It took {} seconds to complete all steps.", state.time)
}
