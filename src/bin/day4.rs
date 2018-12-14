extern crate chrono;
extern crate regex;

use chrono::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::fs;
use std::num::ParseIntError;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug)]
enum Activity {
    BeginShift(usize),
    FallsAsleep,
    WakesUp
}

#[derive(Debug)]
struct Entry {
    datetime: DateTime<Utc>,
    activity: Activity
}

impl FromStr for Activity {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "falls asleep" => Ok(Activity::FallsAsleep),
            "wakes up" => Ok(Activity::WakesUp),
            _ => {
                let reg = Regex::new(r"Guard #(\d+) begins shift").unwrap();
                let cap = reg.captures(s).unwrap();
                cap[1].parse::<usize>().map(|id| Activity::BeginShift(id))
            }
        }
    }
}

impl Eq for Entry {}

impl Ord for Entry {
    fn cmp(&self, other: &Entry) -> Ordering {
        self.datetime.cmp(&other.datetime)
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Entry) -> bool {
        self.datetime == other.datetime
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn make_histogram(ranges: &Vec<Range<u32>>) -> [u32; 60] {
    let mut histogram = [0; 60];
    for range in ranges.iter() {
        for t in range.start..range.end {
            histogram[t as usize] += 1;
        }
    }
    histogram
}

fn main() {
    let input = fs::read_to_string("resources/day4.input")
        .expect("Input file is missing.");

    let reg = Regex::new(r"\[([a-zA-Z0-9-:\s]+)\]\s([\w\s#]+)").unwrap();

    let mut entries : Vec<Entry> = input
        .lines()
        .map(|l| {
            let cap = reg.captures(l).unwrap();
            Entry {
                datetime: Utc.datetime_from_str(&cap[1], "%Y-%m-%d %H:%M").unwrap(),
                activity: Activity::from_str(&cap[2]).unwrap()
            }
        })
        .collect();

    entries.sort();

    let mut sleep_schedule : HashMap<usize, Vec<Range<u32>>> = HashMap::new();
    let mut guard_id = 0;
    for entry in entries.iter() {
        match entry.activity {
            Activity::BeginShift(id) => {
                sleep_schedule.entry(id).or_insert(Vec::new());
                guard_id = id;
            },
            Activity::FallsAsleep => sleep_schedule.get_mut(&guard_id).unwrap().push(entry.datetime.time().minute()..std::u32::MAX),
            Activity::WakesUp => sleep_schedule.get_mut(&guard_id).unwrap().last_mut().unwrap().end = entry.datetime.time().minute(),
        }
    }

    let sleepiest_guard = sleep_schedule
        .iter()
        .map(|(id, ranges)| (*id, ranges.iter().fold(0, |a, b| a + b.end - b.start - 1)))
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    println!("The sleepiest guard is #{}", sleepiest_guard.0);

    let likeliest_time = sleep_schedule
        .get(&sleepiest_guard.0)
        .map(make_histogram)
        .map(|histogram| {
            histogram.iter()
                .enumerate()
                .max_by(|(_,a),(_,b)| a.cmp(b))
                .unwrap()
                .0
        })
        .unwrap();

    println!("The likeliest minute the guard will be asleep is at {}", likeliest_time);
    println!("The resulting answer is {}", sleepiest_guard.0 * likeliest_time);

    let (most_consistent_sleeper, (most_consistent_time, _)) = sleep_schedule
        .iter()
        .map(|(id, ranges)| (
            *id,
            make_histogram(&ranges)
                .iter()
                .enumerate()
                .max_by(|(_,a),(_,b)| a.cmp(b))
                .map(|(a,b)| (a,*b))
                .unwrap()
        ))
        .max_by(|(_,(_,a)),(_,(_,b))| a.cmp(b))
        .unwrap();


    let strategy2 = most_consistent_sleeper * most_consistent_time;
    println!("Guard #{} is the most consistent sleeper at minute {}, resulting in {}.", most_consistent_sleeper, most_consistent_time, strategy2);
}