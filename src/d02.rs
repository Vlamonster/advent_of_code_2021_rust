pub fn p1(input: &str) -> String {
    let (mut x, mut y) = (0, 0);
    for line in input.lines() {
        let (direction, length) = line.split_once(' ').unwrap();
        let length = length.parse::<isize>().unwrap();
        match direction {
            "forward" => x += length,
            "down" => y += length,
            "up" => y -= length,
            _ => unreachable!(),
        }
    }
    format!("{}", x * y)
}

pub fn p2(input: &str) -> String {
    let (mut x, mut y, mut aim) = (0, 0, 0);
    for line in input.lines() {
        let (direction, length) = line.split_once(' ').unwrap();
        let length = length.parse::<isize>().unwrap();
        match direction {
            "forward" => {
                x += length;
                y += aim * length;
            },
            "down" => aim += length,
            "up" => aim -= length,
            _ => unreachable!(),
        }
    }
    format!("{}", x * y)
}

#[test]
fn p1_example() {
    assert_eq!(p1(include_str!("../inputs/d02_example.txt")), "150");
}

#[test]
fn p1_real() {
    assert_eq!(p1(include_str!("../inputs/d02.txt")), "1635930");
}

#[test]
fn p2_example() {
    assert_eq!(p2(include_str!("../inputs/d02_example.txt")), "900");
}

#[test]
fn p2_real() {
    assert_eq!(p2(include_str!("../inputs/d02.txt")), "1781819478");
}
