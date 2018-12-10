use std::collections::HashMap;
use std::str::Chars;

fn counts(xs: Chars) -> HashMap<char, usize> {
    let mut tally = HashMap::new();
    for x in xs {
        *tally.entry(x).or_insert(0) += 1;
    }
    tally
}

fn pair_that_differ_by(xs: &[String], count: usize) -> Option<(String, String)> {
    for a in xs {
        for b in xs {
            if a.chars().zip(b.chars()).filter(|(ai, bj)| ai != bj).count() == count {
                return Some((a.clone(), b.clone()));
            }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;
    use std::fs;

    lazy_static! {
        static ref INPUTS: Vec<String> = fs::read_to_string("data/day2/input")
            .expect("read input file")
            .lines()
            .map(|s| String::from(s))
            .collect();
    }

    #[test]
    fn part1() {
        let has2 = INPUTS
            .iter()
            .filter(|s| counts(s.chars()).values().any(|&c| c == 2))
            .count();
        let has3 = INPUTS
            .iter()
            .filter(|s| counts(s.chars()).values().any(|&c| c == 3))
            .count();
        assert_eq!(has2 * has3, 7134);
    }

    #[test]
    fn part2() {
        let (a, b) = pair_that_differ_by(&INPUTS, 1).unwrap();
        let common: String = a
            .chars()
            .zip(b.chars())
            .filter_map(|(ai, bj)| if ai == bj { Some(ai) } else { None })
            .collect();
        assert_eq!(common, "kbqwtcvzhmhpoelrnaxydifyb");
    }
}
