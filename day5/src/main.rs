use std::fs::File;
use std::io::prelude::*;

static T1: &str = "aA";
static T2: &str = "abBA";
static T3: &str = "abAB";
static T4: &str = "aabAAB";
static T5: &str = "dabAcCaCBAcCcaDA";

fn is_inverse_polarity(u1:char, u2: char) -> bool {
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

fn part1(input: &str) -> String {
    let mut polymers = String::from(input);
    let mut reacted: String = polymers.chars().fold(Vec::new(), react).iter().collect();

    while polymers != reacted {
        polymers = reacted;
        reacted = polymers.chars().fold(Vec::new(), react).iter().collect();
    };
    reacted
}

fn main() {
    let mut f = File::open("resources/input").expect("file not found");
    let mut input = String::new();
    f.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    let p1 = part1(T1);
    println!("{}: \"{}\" = \"{}\" - {}", T1, p1, "", p1.len());
    let p2 = part1(T2);
    println!("{}: \"{}\" = \"{}\" - {}", T2, p2, "", p2.len());
    let p3 = part1(T3);
    println!("{}: \"{}\" = \"{}\" - {}", T3, p3, "abAB", p3.len());
    let p4 = part1(T4);
    println!("{}: \"{}\" = \"{}\" - {}", T4, p4, "aabAAB", p4.len());
    let p5 = part1(T5);
    println!(
        "{}: \"{}\" = \"{}\" - {}",
        T5,
        p5,
        "dabCBAcaDA",
        p5.len()
    );
    println!("{}", input.len());
    println!("Answer: {}", part1(&input.trim()).len());
}
