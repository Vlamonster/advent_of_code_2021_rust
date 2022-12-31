use itertools::Itertools;

fn least_common(nums: &Vec<usize>, bit: usize) -> usize {
    usize::from(nums.iter().filter(|&x| x & bit == 0).count() > nums.len() / 2)
}

fn most_common(nums: &Vec<usize>, bit: usize) -> usize {
    usize::from(nums.iter().filter(|&x| x & bit == 0).count() <= nums.len() / 2)
}

pub fn p1(input: &str) -> String {
    let nums = input
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect_vec();
    let length = input.lines().next().unwrap().len();
    let mut gamma = 0;
    let mut epsilon = 0;
    for index in 0..length {
        gamma |= most_common(&nums, 1 << index) << index;
        epsilon |= least_common(&nums, 1 << index) << index;
    }
    (gamma * epsilon).to_string()
}

pub fn p2(input: &str) -> String {
    let nums = input
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect_vec();
    let length = input.lines().next().unwrap().len();
    let oxy_candidates = (0..length).rev().fold(nums.clone(), |acc, index| {
        if acc.len() == 1 {
            return acc;
        }
        let oxy_criteria = most_common(&acc, 1 << index);
        acc.iter()
            .cloned()
            .filter(|&y| y & (1 << index) == oxy_criteria << index)
            .collect_vec()
    });
    let co2_candidates = (0..length).rev().fold(nums, |acc, index| {
        if acc.len() == 1 {
            return acc;
        }
        let co2_criteria = least_common(&acc, 1 << index);
        acc.iter()
            .cloned()
            .filter(|&y| y & (1 << index) == co2_criteria << index)
            .collect_vec()
    });
    (oxy_candidates[0] * co2_candidates[0]).to_string()
}

#[test]
fn p1_example() {
    assert_eq!(p1(include_str!("../inputs/d03_example.txt")), "198");
}

#[test]
fn p1_real() {
    assert_eq!(p1(include_str!("../inputs/d03.txt")), "3901196");
}

#[test]
fn p2_example() {
    assert_eq!(p2(include_str!("../inputs/d03_example.txt")), "230");
}

#[test]
fn p2_real() {
    assert_eq!(p2(include_str!("../inputs/d03.txt")), "4412188");
}
