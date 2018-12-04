extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

static TEST_CASE: &str = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Coord(u32, u32);

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct ClaimWithCoords {
    id: u32,
    coords: Vec<Coord>,
}

fn claim_to_coords(claim: Claim) -> ClaimWithCoords {
    let mut coords = Vec::new();
    for left in claim.left..(claim.left + claim.width) {
        for top in claim.top..(claim.top + claim.height) {
            coords.push(Coord(left, top))
        }
    }
    ClaimWithCoords {
        id: claim.id,
        coords: coords,
    }
}

fn create_claims(claim_desc: &str) -> Vec<ClaimWithCoords> {
    let claim_re: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let captures = claim_re.captures_iter(claim_desc);

    captures
        .map(|claim| Claim {
            id: claim.get(1).unwrap().as_str().parse::<u32>().unwrap(),
            left: claim.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            top: claim.get(3).unwrap().as_str().parse::<u32>().unwrap(),
            width: claim.get(4).unwrap().as_str().parse::<u32>().unwrap(),
            height: claim.get(5).unwrap().as_str().parse::<u32>().unwrap(),
        })
        .map(claim_to_coords)
        .collect()
}

// The problem is that many of the claims overlap, causing two or more claims to
// cover part of the same areas. For example, consider the following claims:

// #1 @ 1,3: 4x4
// #2 @ 3,1: 4x4
// #3 @ 5,5: 2x2

// If the Elves all proceed with their own plans, none of them will have enough
// fabric. How many square inches of fabric are within two or more claims?

fn part1(claims: &Vec<ClaimWithCoords>) -> usize {
    let mut seen = HashSet::new();
    let mut collisions = HashSet::new();
    // .iter() creates a borrowed iterable so
    // we don't move claims and claims.coords
    for claim in claims.iter() {
        for coord in claim.coords.iter() {
            if seen.contains(coord) {
                collisions.insert(coord);
            } else {
                seen.insert(coord);
            }
        }
    }
    collisions.len()
}

// What is the ID of the only claim that doesn't overlap?

type Grid = HashMap<Coord, HashSet<u32>>;

fn populate_grid(claims: &Vec<ClaimWithCoords>) -> Grid {
    let mut grid: Grid = HashMap::new();
    for claim in claims.iter() {
        for coord in claim.coords.iter() {
            // get the entry of the grid at coord, or else insert new HashSet
            let ids = grid.entry(*coord).or_insert(HashSet::new());
            ids.insert(claim.id);
        }
    }
    grid
}

fn part2(claims: &Vec<ClaimWithCoords>) -> u32 {
    println!("Populating grid..");
    let grid = populate_grid(claims);
    println!("Finding the non-duplicate..");

    // Gross mutable for loops inc
    let mut lonely = 0;
    for claim in claims.iter() {
        let mut is_lonely = true;
        for coord in claim.coords.iter() {
            let ids = grid.get(coord).unwrap();
            if ids.len() != 1 {
                is_lonely = false;
                break;
            }
        }
        if is_lonely {
            lonely = claim.id;
            break;
        }
    }
    lonely
}

fn main() {
    let mut f = File::open("resources/input").expect("File not found");
    let mut input = String::new();

    f.read_to_string(&mut input).expect("Error reading file");

    let test_case = create_claims(TEST_CASE);
    // &*input is an "explicit borrow" that allows us to convert String -> &str
    let claims = create_claims(&*input);

    println!("-- Part1 --");
    println!("How many square inches of fabric are within two or more claims?");
    println!("Test case: {} = {}", part1(&test_case), 4);
    println!("Answer: {}", part1(&claims));
    println!("-- Part2 --");
    println!("What is the ID of the only claim that doesn't overlap?");
    println!("Test case: {} = {}", part2(&test_case), 3);
    println!("Answer: {}", part2(&claims));
}
