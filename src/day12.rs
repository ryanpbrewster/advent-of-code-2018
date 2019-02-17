use std::collections::HashSet;

#[derive(Clone)]
struct State {
    alive: HashSet<i32>,
    rules: HashSet<Vec<bool>>,
}

impl State {
    fn step(&mut self) {
        let mut next = HashSet::new();
        let lo = self.alive.iter().min().unwrap_or(&0);
        let hi = self.alive.iter().max().unwrap_or(&0);
        for i in (lo - 4)..=(hi + 4) {
            let neighborhood: Vec<bool> =
                (i - 2..=i + 2).map(|j| self.alive.contains(&j)).collect();
            if self.rules.contains(&neighborhood) {
                next.insert(i);
            }
        }
        self.alive = next;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INIT: Vec<u8> = b"##.#.#.##..#....######..#..#...#.#..#.#.#..###.#.#.#..#..###.##.#..#.##.##.#.####..##...##..#..##.#.".to_vec();
        static ref RULES: Vec<Vec<u8>> = vec![
            b"...##",
            b".#...",
            b".#..#",
            b".#.##",
            b".###.",
            b".####",
            b"#..#.",
            b"#..##",
            b"#.#.#",
            b"#.##.",
            b"##...",
            b"##..#",
            b"##.##",
            b"###.."
        ].into_iter().map(|s| s.to_vec()).collect();
        static ref INPUT: State = State {
            alive: INIT
                .iter()
                .enumerate()
                .filter_map(|(i, &b)| if b == b'#' { Some(i as i32) } else { None })
                .collect(),
            rules: RULES.iter().map(|r| r.iter().map(|&b| b == b'#').collect()).collect()
        };
    }

    #[test]
    fn part1() {
        let mut state = INPUT.clone();
        for _ in 0..20 {
            state.step();
        }
        assert_eq!(state.alive.iter().sum::<i32>(), 2140);
    }
}
