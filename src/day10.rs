use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Star {
    r: [i32; 2],
    v: [i32; 2],
}
lazy_static! {
    // Example: position=< 52484, -20780> velocity=<-5,  2>
    static ref PATTERN: Regex = Regex::new(r"position=<\s*(?P<rx>-?\d+),\s*(?P<ry>-?\d+)> velocity=<\s*(?P<vx>-?\d+),\s*(?P<vy>-?\d+)>").unwrap();
}
impl FromStr for Star {
    type Err = Box<dyn ::std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cap = PATTERN
            .captures(s)
            .ok_or(format!("could not parse {}", s))?;
        Ok(Star {
            r: [cap["rx"].parse()?, cap["ry"].parse()?],
            v: [cap["vx"].parse()?, cap["vy"].parse()?],
        })
    }
}

impl Star {
    fn step(&mut self, n: i32) {
        self.r[0] += n * self.v[0];
        self.r[1] += n * self.v[1];
    }
}

fn bounding_box(stars: &[Star]) -> [RangeInclusive<i32>; 2] {
    let x0 = stars.iter().map(|s| s.r[0]).min().unwrap();
    let x1 = stars.iter().map(|s| s.r[0]).max().unwrap();
    let y0 = stars.iter().map(|s| s.r[1]).min().unwrap();
    let y1 = stars.iter().map(|s| s.r[1]).max().unwrap();
    [x0..=x1, y0..=y1]
}

fn objective(stars: &[Star]) -> u32 {
    let [x, y] = bounding_box(stars);
    (x.end() - x.start()) as u32 + (y.end() - y.start()) as u32
}

fn pretty_print(stars: &[Star]) -> String {
    let mut buf = String::new();
    let [xx, yy] = bounding_box(stars);
    let occupied: HashSet<[i32; 2]> = stars.iter().map(|s| s.r).collect();
    for y in yy {
        for x in xx.clone() {
            if occupied.contains(&[x, y]) {
                buf.push('x');
            } else {
                buf.push(' ');
            }
        }
        buf.push('\n');
    }
    buf
}

fn optimize(stars: &mut [Star]) -> usize {
    let mut obj = objective(&stars);
    for i in 0.. {
        stars.iter_mut().for_each(|s| s.step(1));
        let new_obj = objective(&stars);
        if new_obj > obj {
            stars.iter_mut().for_each(|s| s.step(-1));
            return i;
        }
        obj = new_obj;
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use std::io::BufRead;

    lazy_static! {
        static ref INPUT: Vec<Star> = {
            let raw = fs::read("data/day10/input").expect("read input file");
            raw.lines()
                .map(|line| line.unwrap().parse().unwrap())
                .collect()
        };
    }

    #[test]
    fn part1() {
        let mut stars = INPUT.clone();
        optimize(&mut stars);
        assert_eq!(
            pretty_print(&stars).trim(),
            r#"
x    x  xxxxx   xxxxx     xx    x       xxxxxx  xxxxx   x    x
xx   x  x    x  x    x   x  x   x            x  x    x  x    x
xx   x  x    x  x    x  x    x  x            x  x    x  x    x
x x  x  x    x  x    x  x    x  x           x   x    x  x    x
x x  x  xxxxx   xxxxx   x    x  x          x    xxxxx   xxxxxx
x  x x  x    x  x  x    xxxxxx  x         x     x       x    x
x  x x  x    x  x   x   x    x  x        x      x       x    x
x   xx  x    x  x   x   x    x  x       x       x       x    x
x   xx  x    x  x    x  x    x  x       x       x       x    x
x    x  xxxxx   x    x  x    x  xxxxxx  xxxxxx  x       x    x
            "#
            .trim()
        )
    }

    #[test]
    fn part2() {
        let mut stars = INPUT.clone();
        assert_eq!(optimize(&mut stars), 10454);
    }
}
