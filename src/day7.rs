use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::str::FromStr;

struct Dependency {
    before: String,
    after: String,
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
            before: String::from(&cap["before"]),
            after: String::from(&cap["after"]),
        })
    }
}

fn topo_sort(dependencies: &[Dependency]) -> Vec<String> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    for dep in dependencies {
        // Add the edge to the graph.
        graph
            .entry(dep.before.clone())
            .or_default()
            .push(dep.after.clone());
        // Add up the in-degrees
        in_degree.entry(dep.before.clone()).or_default();
        *in_degree.entry(dep.after.clone()).or_default() += 1;
    }

    let mut frontier: BinaryHeap<Reverse<String>> = BinaryHeap::new();
    for (id, &count) in in_degree.iter() {
        if count == 0 {
            frontier.push(Reverse(id.clone()));
        }
    }

    let mut ordered = Vec::new();
    while let Some(Reverse(cur)) = frontier.pop() {
        ordered.push(cur.clone());
        for targets in graph.get(&cur) {
            for target in targets {
                let count = in_degree.get_mut(target).unwrap();
                *count -= 1;
                if *count == 0 {
                    frontier.push(Reverse(target.clone()));
                }
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
    fn part1() {
        let order = topo_sort(&INPUT);
        assert_eq!(
            order.into_iter().collect::<String>(),
            "IOFSJQDUWAPXELNVYZMHTBCRGK"
        );
    }
}
