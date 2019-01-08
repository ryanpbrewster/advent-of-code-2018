struct Node {
    metadata: Vec<usize>,
    children: Vec<Node>,
}
impl Node {
    fn from_iter<I>(value: &mut I) -> Option<Node>
    where
        I: Iterator<Item = usize>,
    {
        let num_children = value.next()?;
        let num_metadata = value.next()?;
        let mut children = Vec::new();
        for _ in 0..num_children {
            children.push(Node::from_iter(value)?);
        }
        let mut metadata = Vec::new();
        for _ in 0..num_metadata {
            metadata.push(value.next()?);
        }
        Some(Node { metadata, children })
    }

    fn simple_sum(&self) -> usize {
        self.metadata.iter().sum::<usize>()
            + self.children.iter().map(|c| c.simple_sum()).sum::<usize>()
    }

    fn complex_sum(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            let rec: Vec<usize> = self.children.iter().map(|c| c.complex_sum()).collect();
            self.metadata
                .iter()
                .filter_map(|idx| rec.get(idx - 1))
                .sum()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;
    use std::fs;

    lazy_static! {
        static ref INPUT: Vec<usize> = {
            let raw = fs::read_to_string("data/day8/input").expect("read input file");
            raw.split_whitespace().map(|w| w.parse().unwrap()).collect()
        };
    }

    #[test]
    fn smoke() {
        let input = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
        let mut iter = input.iter().cloned();
        let node = Node::from_iter(&mut iter).unwrap();

        assert_eq!(node.simple_sum(), 138);
    }

    #[test]
    fn part1() {
        let mut iter = INPUT.iter().cloned();
        let node = Node::from_iter(&mut iter).unwrap();
        assert_eq!(node.simple_sum(), 42254);
    }

    #[test]
    fn part2() {
        let mut iter = INPUT.iter().cloned();
        let node = Node::from_iter(&mut iter).unwrap();
        assert_eq!(node.complex_sum(), 25007);
    }
}
