use std::collections::HashSet;

fn find_first_duplicate(xs: &[i32]) -> i32 {
    let mut cur = 0;
    let mut seen = HashSet::new();
    loop {
        for x in xs {
            if seen.contains(&cur) {
                return cur;
            }
            seen.insert(cur);
            cur += x;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;
    use std::fs;

    lazy_static! {
        static ref INPUTS: Vec<i32> = fs::read_to_string("data/day1/input")
            .expect("read input file")
            .lines()
            .map(|s| s.parse().expect("parse line to i32"))
            .collect();
    }

    #[test]
    fn part1() {
        assert_eq!(INPUTS.iter().cloned().sum::<i32>(), 402);
    }

    #[test]
    fn part2() {
        assert_eq!(find_first_duplicate(&INPUTS), 481);
    }
}
