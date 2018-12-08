static TEST_CASE: &str = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

static INPUT: &str = include_str!("../resources/input");

fn mh_distance((p1, p2): (i32, i32), (q1, q2): (i32, i32)) -> i32 {
    (p1 - q1) + (p2 - q2)
}

fn main() {
    println!("Hello, world!");
}
