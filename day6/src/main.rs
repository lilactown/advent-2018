extern crate prettytable;
extern crate regex;

use prettytable::{Cell, Row, Table};
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

static TEST_CASE: &str = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

static INPUT: &str = include_str!("../resources/input");

fn mh_distance((p1, p2): (i32, i32), (q1, q2): (i32, i32)) -> i32 {
    (p1 - q1).abs() + (p2 - q2).abs()
}

type Point = (char, i32, i32);

#[derive(Debug, Copy, Clone)]
struct Marked {
    color: char,
    top: i32,
    left: i32,
    weight: i32,
}

#[derive(Debug, Clone)]
struct Grid {
    points: Vec<Marked>,
    width: i32,
    height: i32,
}

fn mark_color(left: i32, top: i32, point: Point) -> Marked {
    let (color, l, t) = point;
    Marked {
        color: if t == top && l == left {
            color
        } else {
            color.to_ascii_lowercase()
        },
        top: top,
        left: left,
        weight: mh_distance((t, l), (top, left)),
    }
}

#[derive(Debug)]
enum MWCErr {
    MultipleWeights(i32),
    None,
}

fn min_weight_color(result: Result<Marked, MWCErr>, mark: Marked) -> Result<Marked, MWCErr> {
    match result {
        Err(MWCErr::None) => Ok(mark),
        Ok(m) if m.weight == mark.weight => Err(MWCErr::MultipleWeights(m.weight)),
        Ok(m) if m.weight > mark.weight => Ok(mark),
        Err(MWCErr::MultipleWeights(weight)) if weight > mark.weight => Ok(mark),
        _ => result,
    }
}

fn create_grid(points: Vec<Point>) -> (Grid, impl Fn() -> usize) {
    let (_, width, _) = points.iter().max_by_key(|(_, left, _)| left).unwrap();
    let (_, _, height) = points.iter().max_by_key(|(_, _, top)| top).unwrap();

    let mut table = Table::new();

    println!("width: {}, height: {}", width, height);

    let mut g = Vec::new();
    for top in 0..*height + 1 {
        let mut row = Vec::new();
        for left in 0..*width + 1 {
            let marks = points
                .iter()
                .map(|point| mark_color(left, top, *point))
                .fold(Result::Err(MWCErr::None), min_weight_color);
            let mark = match marks {
                Ok(m) => m,
                Err(MWCErr::MultipleWeights(w)) => Marked {
                    color: '.',
                    top: top,
                    left: left,
                    weight: w,
                },
                _ => panic!("Something went wrong while picking the correct marker"),
            };
            row.push(Cell::new(&mark.color.to_string()));
            g.push(mark)
        }
        table.add_row(Row::new(row));
    }
    let print = move || table.printstd();
    (
        Grid {
            points: g,
            width: *width,
            height: *height,
        },
        print,
    )
}

fn create_points(input: &str) -> Vec<Point> {
    let point_re: Regex = Regex::new(r"(\d+), (\d+)").unwrap();
    let mut points: Vec<Point> = Vec::new();
    let mut color = 'A';
    for p in point_re.captures_iter(input) {
        let left = p[1].parse().unwrap();
        let top = p[2].parse().unwrap();
        points.push((color, left, top));
        color = std::char::from_u32((color as u32) + 1).unwrap();
    }
    points
}

fn is_finite<'r>(grid: Grid) -> impl FnMut(&(&char, &Vec<&Marked>)) -> bool {
    let width = grid.width;
    let height = grid.height;
    move |(_, region)| {
        !region.iter().any(|mark| {
            (mark.left == 0 || mark.top == 0 || mark.left == width || mark.top == height)
        })
    }
}

fn regions<'r>(points: Vec<Point>, grid: &'r Grid) -> HashMap<char, Vec<&'r Marked>> {
    let mut regions = HashMap::new();

    for p in points {
        let (color, _, _) = p;
        let region: Vec<&Marked> = grid
            .points
            .iter()
            .filter(|m| m.color.to_ascii_lowercase() == color.to_ascii_lowercase())
            .collect();
        regions.insert(color, region);
    }
    regions
}

fn main() {
    println!("-- part1 --");
    let test_points = create_points(TEST_CASE);

    let (test_grid, _) = create_grid(test_points.clone());

    // prn_table();

    // separate them into regions
    let tgc = &test_grid.clone();
    let test_regions = regions(test_points, tgc);

    let test_finite_regions: HashSet<(&char, usize)> = test_regions
        .iter()
        .filter(is_finite(test_grid))
        .map(|(color, region)| (color, region.len()))
        .collect();

    println!("Test case: {:?}", test_finite_regions);

    let points = create_points(INPUT.trim());

    let (grid, _) = create_grid(points.clone());

    // prn_table();

    // separate them into regions
    let gc = &grid.clone();
    let regions = regions(points, gc);

    let finite_regions: Vec<(&char, usize)> = regions
        .iter()
        .filter(is_finite(grid))
        .map(|(color, region)| (color, region.len()))
        .collect();

    println!("Answer: {:?}", finite_regions.max_by_key(|(_, size)| size));
}
