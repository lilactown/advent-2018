use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn part1(input: Vec<i32>) -> i32 {
    // I could use .sum() but I wanted to get used to .fold
    input.iter().fold(0, |acc, x| acc + x)
}

fn part2(input: Vec<i32>) -> i32 {
    // Not sure what clone is yet, but this gives us an infinite iterable
    let mut input_cycle = input.iter().cloned().cycle();
    let mut seen = HashSet::new();
    let mut current = 0;
    while !seen.contains(&current) {
        seen.insert(current);
        // it's an infinite sequence so we should never not have a value
        current = current + input_cycle.next().unwrap();
    }
    current
}

// I can't figure out how to have solve_fn take an iterable instead of a vec
fn solve<F>(solve_fn: F) -> Result<i32, std::io::Error>
where
    F: Fn(Vec<i32>) -> i32,
{
    let f = File::open("input")?;
    let file = BufReader::new(&f);
    let nums = file
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    Ok(solve_fn(nums))
}

fn main() {
    println!("-- Part1 --");
    match solve(part1) {
        Ok(v) => println!("Answer: {}", v),
        Err(e) => println!("Error: {}", e),
    };
    println!("-- Part2 --");
    match solve(part2) {
        Ok(v) => println!("Answer: {}", v),
        Err(e) => println!("Error: {}", e),
    }
}
