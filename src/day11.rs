use std::ops::Range;

fn power_level(x: i32, y: i32, serial_number: i32) -> i32 {
    let id = x + 10;
    let power = (id * y + serial_number) * id;
    (power % 1_000) / 100 - 5
}

fn total_power_level(xx: Range<i32>, yy: Range<i32>, serial_number: i32) -> i32 {
    let mut total = 0;
    for x in xx {
        for y in yy.clone() {
            total += power_level(x, y, serial_number);
        }
    }
    total
}

#[derive(Debug, Eq, PartialEq)]
struct Square {
    x: i32,
    y: i32,
}

fn largest_total_power(serial_number: i32) -> Square {
    let mut best_square = Square { x: 0, y: 0 };
    let mut best_score = 0;
    for x in 1..=298 {
        for y in 1..=298 {
            let score = total_power_level(x..x + 3, y..y + 3, serial_number);
            if score > best_score {
                best_score = score;
                best_square = Square { x, y };
            }
        }
    }

    best_square
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn smoke() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
        assert_eq!(largest_total_power(42), Square { x: 21, y: 61 });
    }

    #[test]
    fn part1() {
        assert_eq!(largest_total_power(7400), Square { x: 34, y: 72 });
    }
}
