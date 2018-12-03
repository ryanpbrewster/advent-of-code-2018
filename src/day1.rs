fn main() {
    println!("Hello, World!");
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::io::Read;
    use std::path::Path;

    #[test]
    fn part1() {
        let contents = fs::read_to_string("data/day1/input").expect("read input file");
        let total: i32 = contents
            .lines()
            .map(|s| s.parse::<i32>().expect("parse line to i32"))
            .sum();
        assert_eq!(total, 402);
    }
}
