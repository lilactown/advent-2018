use std::fs::File;
use std::io::prelude::*;

static T1: &str = "aA";
static T2: &str = "abBA";
static T3: &str = "abAB";
static T4: &str = "aabAAB";
static T5: &str = "dabAcCaCBAcCcaDA";

static UNIT_TYPES: &str = "abcdefghijklmnopqrstuvwxyz";

fn is_inverse_polarity(u1: char, u2: char) -> bool {
    u1 != u2 && u1.to_ascii_lowercase() == u2.to_ascii_lowercase()
}

fn react(polymers: Vec<char>, unit: char) -> Vec<char> {
    let mut reacted = polymers.clone();
    match polymers.last() {
        Some(u) if is_inverse_polarity(*u, unit) => {
            reacted.pop();
            ()
        }
        _ => reacted.push(unit),
    };
    reacted
}

// The polymer is formed by smaller units which, when triggered, react with each
// other such that two adjacent units of the same type and opposite polarity are
// destroyed. Units' types are represented by letters; units' polarity is
// represented by capitalization. For instance, r and R are units with the same
// type but opposite polarity, whereas r and s are entirely different types and
// do not react.

// How many units remain after fully reacting the polymer you scanned?

fn part1(input: &str) -> String {
    let mut polymers = String::from(input);
    let mut reacted: String = polymers.chars().fold(Vec::new(), react).iter().collect();

    while polymers != reacted {
        polymers = reacted;
        reacted = polymers.chars().fold(Vec::new(), react).iter().collect();
    }
    reacted
}

// What is the length of the shortest polymer you can produce by removing all
// units of exactly one type and fully reacting the result?

fn part2(input: &str) -> usize {
    let mut sizes = Vec::new();
    for unit_type in UNIT_TYPES.chars() {
        let reacted = part1(
            &input
                .replace(unit_type, "")
                .replace(unit_type.to_ascii_uppercase(), ""),
        );
        sizes.push(reacted.len())
    }
    *sizes.iter().min().unwrap()
}

fn main() {
    let mut f = File::open("resources/input").expect("file not found");
    let mut input = String::new();
    f.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    println!("-- Part1 --");
    let p1 = part1(T1);
    println!("{}: \"{}\" = \"{}\" - {}", T1, p1, "", p1.len());
    let p2 = part1(T2);
    println!("{}: \"{}\" = \"{}\" - {}", T2, p2, "", p2.len());
    let p3 = part1(T3);
    println!("{}: \"{}\" = \"{}\" - {}", T3, p3, "abAB", p3.len());
    let p4 = part1(T4);
    println!("{}: \"{}\" = \"{}\" - {}", T4, p4, "aabAAB", p4.len());
    let p5 = part1(T5);
    println!("{}: \"{}\" = \"{}\" - {}", T5, p5, "dabCBAcaDA", p5.len());
    println!("Answer: {}", part1(&input.trim()).len());

    println!("-- Part2 --");
    let sp5 = part2(T5);
    println!("{}: \"{}\" = \"{}\" - {}", T5, sp5, "daDA", sp5);
    println!("Answer: {}", part2(&input.trim()));
}
