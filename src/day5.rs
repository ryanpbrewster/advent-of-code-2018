fn compact(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i + 1 < chars.len() {
        if chars[i].to_ascii_lowercase() == chars[i + 1].to_ascii_lowercase()
            && chars[i] != chars[i + 1]
        {
            chars.remove(i);
            chars.remove(i);
            if i > 0 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }
    chars.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;
    use std::fs;

    lazy_static! {
        static ref INPUT: String = fs::read_to_string("data/day5/input").expect("read input file");
    }

    #[test]
    fn part1() {
        assert_eq!(compact(INPUT.trim()).len(), 11118);
    }

    #[test]
    fn part2() {
        let shortest = (b'A'..=b'Z')
            .map(|b| b as char)
            .map(|c| {
                let input = INPUT
                    .trim()
                    .replace(c, "")
                    .replace(c.to_ascii_lowercase(), "");
                compact(&input).len()
            })
            .min()
            .unwrap();
        assert_eq!(shortest, 6948);
    }
}
