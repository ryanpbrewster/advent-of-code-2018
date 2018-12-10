use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Rectangle {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

lazy_static! {
    // Example: #37 @ 801,484: 22x28
    static ref PATTERN: Regex = Regex::new(r"#\d+ @ (?P<x>\d+),(?P<y>\d+): (?P<w>\d+)x(?P<h>\d+)").unwrap();
}
impl FromStr for Rectangle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let cap = PATTERN.captures(s).ok_or(())?;
        Ok(Rectangle {
            x: cap["x"].parse().map_err(|_| ())?,
            y: cap["y"].parse().map_err(|_| ())?,
            w: cap["w"].parse().map_err(|_| ())?,
            h: cap["h"].parse().map_err(|_| ())?,
        })
    }
}

fn coverage(rects: &[Rectangle]) -> HashMap<(u32, u32), usize> {
    let mut tally = HashMap::new();
    for rect in rects {
        for i in 0..rect.h {
            for j in 0..rect.w {
                *tally.entry((rect.x + j, rect.y + i)).or_insert(0) += 1;
            }
        }
    }
    tally
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn part1() {
        let contents = fs::read_to_string("data/day3/input").expect("read input file");
        let rects = contents
            .lines()
            .map(|s| s.parse())
            .collect::<Result<Vec<Rectangle>, _>>()
            .expect("parse inputs");
        let tally = coverage(&rects);
        let x2 = tally.values().filter(|&&c| c >= 2).count();
        assert_eq!(x2, 101196);
    }
}
