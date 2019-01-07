use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Point(i32, i32);
impl Point {
    fn distance(&self, other: &Point) -> usize {
        (self.0 - other.0).abs() as usize + (self.1 - other.1).abs() as usize
    }
    fn neighbors(&self) -> Vec<Point> {
        vec![
            Point(self.0 + 1, self.1),
            Point(self.0, self.1 + 1),
            Point(self.0 - 1, self.1),
            Point(self.0, self.1 - 1),
        ]
    }
}

lazy_static! {
    // Example: 13, 94
    static ref PATTERN: Regex = Regex::new(r"(?P<x>\d+),\s*(?P<y>\d+)").unwrap();
}
impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Point, String> {
        let cap = PATTERN
            .captures(s)
            .ok_or(format!("could not parse {}", s))?;
        Ok(Point(cap["x"].parse().unwrap(), cap["y"].parse().unwrap()))
    }
}

fn regions(points: &[Point]) -> HashMap<Point, usize> {
    let mut areas: HashMap<Point, usize> = HashMap::new();
    for &p in points {
        areas.insert(p, 0);
    }

    let bounding_box = BoundingBox::from(points);
    let mut grid: HashMap<Point, Option<Point>> = HashMap::new();

    struct Step {
        origin: Point,
        cur: Point,
    };
    let mut q = VecDeque::new();
    for &p in points {
        q.push_back(Step { origin: p, cur: p });
    }

    while let Some(Step { origin, cur }) = q.pop_front() {
        if !bounding_box.contains(&cur) {
            continue;
        }
        match grid.entry(cur) {
            ::std::collections::hash_map::Entry::Vacant(vac) => {
                vac.insert(Some(origin));
                *areas.get_mut(&origin).unwrap() += 1;
                for n in cur.neighbors() {
                    q.push_back(Step { origin, cur: n });
                }
            }
            ::std::collections::hash_map::Entry::Occupied(mut occ) => {
                if let Some(prev) = occ.get() {
                    if *prev != origin && cur.distance(prev) == cur.distance(&origin) {
                        *areas.get_mut(prev).unwrap() -= 1;
                        occ.insert(None);
                    }
                }
            }
        }
    }

    for x in bounding_box.x0..=bounding_box.x1 {
        if let Some(Some(closest)) = grid.get(&Point(x, bounding_box.y0)) {
            areas.remove(closest);
        }
        if let Some(Some(closest)) = grid.get(&Point(x, bounding_box.y1)) {
            areas.remove(closest);
        }
    }
    for y in bounding_box.y0..=bounding_box.y1 {
        if let Some(Some(closest)) = grid.get(&Point(bounding_box.x0, y)) {
            areas.remove(closest);
        }
        if let Some(Some(closest)) = grid.get(&Point(bounding_box.x1, y)) {
            areas.remove(closest);
        }
    }

    areas
}

#[derive(Debug)]
struct BoundingBox {
    x0: i32,
    x1: i32,
    y0: i32,
    y1: i32,
}
impl From<Point> for BoundingBox {
    fn from(p: Point) -> Self {
        BoundingBox {
            x0: p.0,
            x1: p.0,
            y0: p.1,
            y1: p.1,
        }
    }
}
impl From<&[Point]> for BoundingBox {
    fn from(points: &[Point]) -> Self {
        let mut bb = BoundingBox::from(points[0]);
        for &p in &points[1..] {
            if p.0 < bb.x0 {
                bb.x0 = p.0;
            }
            if p.0 > bb.x1 {
                bb.x1 = p.0;
            }
            if p.1 < bb.y0 {
                bb.y0 = p.1;
            }
            if p.1 > bb.y1 {
                bb.y1 = p.1;
            }
        }
        bb
    }
}
impl BoundingBox {
    fn contains(&self, p: &Point) -> bool {
        (self.x0..=self.x1).contains(&p.0) && (self.y0..=self.y1).contains(&p.1)
    }
}

fn region_by_predicate<P>(points: &[Point], pred: P) -> HashSet<Point>
where
    P: Fn(Point, &[Point]) -> bool,
{
    // Seed the queue with all the points
    let mut q = VecDeque::new();
    for &p in points {
        q.push_back(p);
    }

    let mut visited = HashSet::new();
    while let Some(cur) = q.pop_front() {
        if visited.contains(&cur) {
            continue;
        }
        if pred(cur, points) {
            visited.insert(cur);
            for n in cur.neighbors() {
                q.push_back(n);
            }
        }
    }
    visited
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;
    use std::fs;
    use std::io::BufRead;

    lazy_static! {
        static ref INPUT: Vec<Point> = {
            let raw = fs::read("data/day6/input").expect("read input file");
            raw.lines()
                .map(|line| line.unwrap().parse().unwrap())
                .collect()
        };
    }

    #[test]
    fn smoke() {
        /*
        aaaaa.cccc
        aAaaa.cccc
        aaaddecccc
        aadddeccCc
        ..dDdeeccc
        bb.deEeecc
        bBb.eeee..
        bbb.eeefff
        bbb.eeffff
        bbb.ffffFf
        */

        let a = Point(1, 1);
        let b = Point(1, 6);
        let c = Point(8, 3);
        let d = Point(3, 4);
        let e = Point(5, 5);
        let f = Point(8, 9);

        let areas = regions(&vec![a, b, c, d, e, f]);
        assert_eq!(areas, vec![(d, 9), (e, 17)].into_iter().collect()); // A, B, C, and F are all infinite in size
    }

    #[test]
    fn part1() {
        let areas = regions(&INPUT);
        assert_eq!(*areas.values().max().unwrap(), 3276);
    }

    #[test]
    fn part2() {
        let region = region_by_predicate(&INPUT, |cur, pts| {
            pts.iter().map(|p| cur.distance(p)).sum::<usize>() < 10_000
        });
        assert_eq!(region.len(), 38380);
    }
}
