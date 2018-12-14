fn compact_length(mut chars: Vec<u8>) -> usize {
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
    chars.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;
    use std::fs;

    lazy_static! {
        static ref INPUT: Vec<u8> = {
            let mut raw = fs::read("data/day5/input").expect("read input file");
            raw.retain(|&b| (b'a' <= b && b <= b'z') || (b'A' <= b && b <= b'Z'));
            raw
        };
    }

    #[test]
    fn part1() {
        assert_eq!(compact_length(INPUT.clone()), 11118);
    }

    #[test]
    fn part2() {
        let shortest = (b'A'..=b'Z')
            .map(|r| {
                let mut input = INPUT.clone();
                input.drain_filter(|&mut b| b == r || b == r.to_ascii_lowercase());
                compact_length(input)
            })
            .min()
            .unwrap();
        assert_eq!(shortest, 6948);
    }
}
