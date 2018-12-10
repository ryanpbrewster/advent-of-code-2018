use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Rectangle {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

lazy_static! {
    // Example: #37 @ 801,484: 22x28
    static ref PATTERN: Regex = Regex::new(r"#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<w>\d+)x(?P<h>\d+)").unwrap();
}
impl FromStr for Rectangle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let cap = PATTERN.captures(s).ok_or(())?;
        Ok(Rectangle {
            id: cap["id"].parse().map_err(|_| ())?,
            x: cap["x"].parse().map_err(|_| ())?,
            y: cap["y"].parse().map_err(|_| ())?,
            w: cap["w"].parse().map_err(|_| ())?,
            h: cap["h"].parse().map_err(|_| ())?,
        })
    }
}

impl Rectangle {
    fn points(&self) -> Points {
        Points {
            rect: self.clone(),
            i: 0,
            j: 0,
        }
    }
}

struct Points {
    rect: Rectangle,
    i: u32,
    j: u32,
}

impl Iterator for Points {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        if self.i >= self.rect.h {
            return None;
        }
        let item = (self.rect.x + self.j, self.rect.y + self.i);
        if self.j + 1 < self.rect.w {
            self.j += 1;
        } else {
            self.j = 0;
            self.i += 1;
        };
        Some(item)
    }
}

fn coverage(rects: &[Rectangle]) -> HashMap<(u32, u32), usize> {
    let mut tally = HashMap::new();
    for rect in rects {
        for pt in rect.points() {
            *tally.entry(pt).or_insert(0) += 1;
        }
    }
    tally
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn points_smoke_test() {
        let r = Rectangle {
            id: 0,
            x: 1,
            y: 2,
            w: 3,
            h: 2,
        };
        assert_eq!(r.points().count(), 3 * 2);
        assert_eq!(
            r.points().collect::<Vec<_>>(),
            vec![(1, 2), (2, 2), (3, 2), (1, 3), (2, 3), (3, 3)]
        );
    }

    lazy_static! {
        static ref INPUTS: Vec<Rectangle> = {
            let contents = fs::read_to_string("data/day3/input").expect("read input file");
            contents
                .lines()
                .map(|s| s.parse())
                .collect::<Result<Vec<Rectangle>, _>>()
                .expect("parse inputs")
        };
    }

    #[test]
    fn part1() {
        let tally = coverage(&INPUTS);
        let x2 = tally.values().filter(|&&c| c >= 2).count();
        assert_eq!(x2, 101196);
    }

    #[test]
    fn part2() {
        let tally = coverage(&INPUTS);
        let no_overlaps = INPUTS
            .iter()
            .filter(|&rect| rect.points().all(|pt| *tally.get(&pt).unwrap_or(&0) <= 1))
            .next();
        assert_eq!(no_overlaps.expect("find a no-overlap rectangle").id, 243);
    }
}
