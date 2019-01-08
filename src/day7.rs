use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::str::FromStr;

struct Dependency {
    before: u8,
    after: u8,
}
lazy_static! {
    // Example: Step B must be finished before step C can begin.
    static ref PATTERN: Regex = Regex::new(r"Step (?P<before>[[:alpha:]]) must be finished before step (?P<after>[[:alpha:]]) can begin.").unwrap();
}
impl FromStr for Dependency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let cap = PATTERN
            .captures(s)
            .ok_or(format!("could not parse {}", s))?;
        Ok(Dependency {
            before: cap["before"].as_bytes()[0],
            after: cap["after"].as_bytes()[0],
        })
    }
}

fn topo_sort(dependencies: &[Dependency]) -> Vec<u8> {
    let mut graph: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut in_degree: HashMap<u8, usize> = HashMap::new();
    for dep in dependencies {
        // Add the edge to the graph.
        graph.entry(dep.before).or_default().push(dep.after);
        // Add up the in-degrees
        in_degree.entry(dep.before).or_default();
        *in_degree.entry(dep.after).or_default() += 1;
    }

    let mut frontier: BinaryHeap<Reverse<u8>> = BinaryHeap::new();
    for (&id, &count) in in_degree.iter() {
        if count == 0 {
            frontier.push(Reverse(id));
        }
    }

    let mut ordered = Vec::new();
    while let Some(Reverse(cur)) = frontier.pop() {
        ordered.push(cur);
        for targets in graph.get(&cur) {
            for &target in targets {
                let count = in_degree.get_mut(&target).unwrap();
                *count -= 1;
                if *count == 0 {
                    frontier.push(Reverse(target));
                }
            }
        }
    }

    ordered
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
struct Event {
    t: usize,
    id: u8,
}
fn timed_topo_sort<F>(dependencies: &[Dependency], mut workers: usize, duration: F) -> Vec<Event>
where
    F: Fn(u8) -> usize,
{
    let mut graph: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut in_degree: HashMap<u8, usize> = HashMap::new();
    for dep in dependencies {
        // Add the edge to the graph.
        graph.entry(dep.before).or_default().push(dep.after);
        // Add up the in-degrees
        in_degree.entry(dep.before).or_default();
        *in_degree.entry(dep.after).or_default() += 1;
    }

    let mut frontier: BinaryHeap<Reverse<u8>> = BinaryHeap::new();
    for (&id, &count) in in_degree.iter() {
        if count == 0 {
            frontier.push(Reverse(id));
        }
    }

    let mut events: BinaryHeap<Reverse<Event>> = BinaryHeap::new();
    let mut ordered: Vec<Event> = Vec::new();

    while workers > 0 {
        if let Some(Reverse(id)) = frontier.pop() {
            workers -= 1;
            events.push(Reverse(Event {
                id,
                t: duration(id),
            }));
        } else {
            break;
        }
    }

    while let Some(Reverse(evt)) = events.pop() {
        workers += 1;
        ordered.push(evt);
        for targets in graph.get(&evt.id) {
            for &target in targets {
                let count = in_degree.get_mut(&target).unwrap();
                *count -= 1;
                if *count == 0 {
                    frontier.push(Reverse(target));
                }
            }
        }

        while workers > 0 {
            if let Some(Reverse(id)) = frontier.pop() {
                workers -= 1;
                events.push(Reverse(Event {
                    id,
                    t: evt.t + duration(id),
                }));
            } else {
                break;
            }
        }
    }

    ordered
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;
    use std::fs;
    use std::io::BufRead;

    lazy_static! {
        static ref INPUT: Vec<Dependency> = {
            let raw = fs::read("data/day7/input").expect("read input file");
            raw.lines()
                .map(|line| line.unwrap().parse().unwrap())
                .collect()
        };
    }

    #[test]
    fn smoke() {
        let deps: Vec<Dependency> = vec![
            ('C', 'A'),
            ('C', 'F'),
            ('A', 'B'),
            ('A', 'D'),
            ('B', 'E'),
            ('D', 'E'),
            ('F', 'E'),
        ]
        .into_iter()
        .map(|(a, b)| Dependency {
            before: a as u8,
            after: b as u8,
        })
        .collect();
        let events = timed_topo_sort(&deps, 2, |id| id as usize - 64);
        assert_eq!(events.last().unwrap().t, 15);
    }

    #[test]
    fn part1() {
        let order = topo_sort(&INPUT);
        assert_eq!(
            String::from_utf8(order).unwrap(),
            "IOFSJQDUWAPXELNVYZMHTBCRGK"
        );
    }

    #[test]
    fn part2() {
        let events = timed_topo_sort(&INPUT, 5, |id| id as usize - 4);
        assert_eq!(events.last().unwrap().t, 931);
    }
}
