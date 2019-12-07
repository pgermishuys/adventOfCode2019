use std::cmp::Ordering;

fn main() {
    let count = (245318..765747).filter(|d| check_part1(*d)).count();
    println!("Part 1: {}", count);

    let count = (245318..765747).filter(|d| check_part2(*d)).count();
    println!("Part 2: {}", count);
}

fn check_part1(mut pw: u32) -> bool {
    let mut repeat = 1;
    let mut double = false;
    let mut previous = pw % 10;
    pw /= 10;

    while pw != 0 {
        let c = pw % 10;
        match c.cmp(&previous) {
            Ordering::Greater => {
                return false;
            }
            Ordering::Equal => {
                repeat += 1;
                if repeat == 2 {
                    double = true;
                }
            }
            Ordering::Less => {
                repeat = 1;
            }
        }
        previous = c;
        pw /= 10;
    }
    double
}

fn check_part2(mut pw: u32) -> bool {
    let mut repeat = 1;
    let mut double = false;
    let mut previous = pw % 10;
    pw /= 10;

    while pw != 0 {
        let c = pw % 10;
        match c.cmp(&previous) {
            Ordering::Greater => {
                return false;
            }
            Ordering::Equal => {
                repeat += 1;
            }
            Ordering::Less => {
                if repeat == 2 {
                    double = true;
                }
                repeat = 1;
            }
        }
        pw /= 10;
        previous = c;
    }
    double || repeat == 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert!(check_part1(111111));
        assert!(!check_part1(223450));
        assert!(!check_part1(123789));
    }

    #[test]
    fn test_examples_part2() {
        assert!(!check_part2(111111));
        assert!(!check_part2(123444));
        assert!(check_part2(112233));
        assert!(check_part2(123445));
        assert!(check_part2(111122));
    }
}
