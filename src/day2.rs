use std::collections::HashMap;
use std::str::Chars;

fn counts(xs: Chars) -> HashMap<char, usize> {
    let mut tally = HashMap::new();
    for x in xs {
        *tally.entry(x).or_insert(0) += 1;
    }
    tally
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn part1() {
        let contents = fs::read_to_string("data/day2/input").expect("read input file");
        let lines: Vec<String> = contents.lines().map(|s| String::from(s)).collect();
        let has2 = lines
            .iter()
            .filter(|s| counts(s.chars()).values().any(|&c| c == 2))
            .count();
        let has3 = lines
            .iter()
            .filter(|s| counts(s.chars()).values().any(|&c| c == 3))
            .count();
        assert_eq!(has2 * has3, 7134);
    }

    #[test]
    fn part2() {
        let contents = fs::read_to_string("data/day2/input").expect("read input file");
        let xyz: i32 = unimplemented!();
        assert_eq!(xyz, 481);
    }
}
