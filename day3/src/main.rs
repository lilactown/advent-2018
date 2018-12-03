extern crate regex;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

static TEST_CASE: &str = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

#[derive(Debug)]
struct Claim {
    id: String,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Coord(u32, u32);

fn create_claims(claim_desc: &str) -> Vec<Claim> {
    let claim_re: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let captures = claim_re.captures_iter(claim_desc);

    captures
        .map(|claim| Claim {
            id: claim.get(1).unwrap().as_str().to_string(),
            left: claim.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            top: claim.get(3).unwrap().as_str().parse::<u32>().unwrap(),
            width: claim.get(4).unwrap().as_str().parse::<u32>().unwrap(),
            height: claim.get(5).unwrap().as_str().parse::<u32>().unwrap(),
        })
        .collect()
}

fn claim_to_coords(claim: Claim) -> Vec<Coord> {
    let mut coords = Vec::new();
    for left in claim.left..(claim.left + claim.width) {
        for top in claim.top..(claim.top + claim.height) {
            coords.push(Coord(left, top))
        }
    }
    coords
}

fn part1(input: &str) -> usize {
    let mut seen = HashSet::new();
    let mut collisions = HashSet::new();
    for claim in create_claims(input) {
        for coord in claim_to_coords(claim) {
            if seen.contains(&coord) {
                collisions.insert(coord);
            } else {
                seen.insert(coord);
            }
        }
    }
    collisions.len()
}

fn main() {
    let mut f = File::open("resources/input").expect("File not found");
    let mut input = String::new();

    f.read_to_string(&mut input).expect("Error reading file");

    println!("-- Part1 --");
    println!("Test case: {} = {}", part1(TEST_CASE), 4);
    // &*input is an "explicit borrow" that allows us to convert String -> &str
    println!("Answer: {}", part1(&*input));
    println!("-- Part2 --");
}
