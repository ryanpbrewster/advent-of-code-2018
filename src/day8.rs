struct Node {
    metadata: Vec<usize>,
    children: Vec<Node>,
}
impl Node {
    fn for_each<F>(&self, f: &mut F) where F: FnMut(&[usize]) {
        f(&self.metadata);
        for child in &self.children {
            child.for_each(f);
        }
    }
    fn from_iter<'a, I>(value: &mut I) -> Result<Self, ()> where I: Iterator<Item = &'a usize> {
        let num_children = *value.next().ok_or(())?;
        let num_metadata = *value.next().ok_or(())?;
        let mut children = Vec::new();
        for _ in 0 .. num_children {
            children.push(Node::from_iter(value)?);
        }
        let mut metadata = Vec::new();
        for _ in 0 .. num_metadata {
            metadata.push(*value.next().ok_or(())?);
        }
        Ok(Node { metadata, children })
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
            raw.split_whitespace()
                .map(|w| w.parse().unwrap())
                .collect()
        };
    }

    #[test]
    fn smoke() {
        let input = vec![2,3,0,3,10,11,12,1,1,0,1,99,2,1,1,2];
        let mut iter = input.iter();
        let node = Node::from_iter(&mut iter).unwrap();

        let mut total = 0;
        node.for_each(&mut |md| total += md.iter().sum::<usize>());
        assert_eq!(total, 138);
    }

    #[test]
    fn part1() {
        let mut iter = INPUT.iter();
        let node = Node::from_iter(&mut iter).unwrap();

        let mut total = 0;
        node.for_each(&mut |md| total += md.iter().sum::<usize>());
        assert_eq!(total, 42254);
    }
}