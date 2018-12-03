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
    use std::fs;

    #[test]
    fn part1() {
        let contents = fs::read_to_string("data/day1/input").expect("read input file");
        let total: i32 = contents
            .lines()
            .map(|s| s.parse::<i32>().expect("parse line to i32"))
            .sum();
        assert_eq!(total, 402);
    }

    #[test]
    fn part2() {
        let contents = fs::read_to_string("data/day1/input").expect("read input file");
        let inputs: Vec<i32> = contents
            .lines()
            .map(|s| s.parse::<i32>().expect("parse line to i32"))
            .collect();
        assert_eq!(find_first_duplicate(&inputs), 481);
    }
}
