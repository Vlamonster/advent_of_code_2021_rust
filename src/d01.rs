use itertools::Itertools;

pub fn p1(input: &str) -> String {
    input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .tuple_windows()
        .filter(|(x1, x2)| x1 < x2)
        .count()
        .to_string()
}

pub fn p2(input: &str) -> String {
    input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .tuple_windows()
        .filter(|(x1, _, _, x4)| x1 < x4)
        .count()
        .to_string()
}

#[test]
fn p1_example() {
    assert_eq!(p1(include_str!("../inputs/d01_example.txt")), "7");
}

#[test]
fn p1_real() {
    assert_eq!(p1(include_str!("../inputs/d01.txt")), "1527");
}

#[test]
fn p2_example() {
    assert_eq!(p2(include_str!("../inputs/d01_example.txt")), "5");
}

#[test]
fn p2_real() {
    assert_eq!(p2(include_str!("../inputs/d01.txt")), "1575");
}
