use std::ops::Range;

fn power_level(x: i32, y: i32, serial_number: i32) -> i32 {
    let id = x + 10;
    let power = (id * y + serial_number) * id;
    (power % 1_000) / 100 - 5
}

#[derive(Debug, Eq, PartialEq)]
struct Square {
    x: i32,
    y: i32,
    size: usize,
}

fn solve(serial_number: i32, grid_size: usize, sizes: Range<usize>) -> (Square, i32) {
    // totals(i, j) = Sum { grid(i', j') | i' <- 1..i, j' <- 1..j }
    let w = grid_size + 1;
    let mut totals = vec![0; w * w];
    for i in 1..=grid_size {
        for j in 1..=grid_size {
            totals[w * i + j] = totals[w * (i - 1) + j] + totals[w * i + (j - 1)]
                - totals[w * (i - 1) + (j - 1)]
                + power_level(j as i32, i as i32, serial_number);
        }
    }

    let mut best_score = 0;
    let mut best_square = Square {
        x: 0,
        y: 0,
        size: 0,
    };
    for size in sizes {
        for x in 1..=w - size {
            for y in 1..=w - size {
                let i0 = y as usize - 1;
                let i1 = y as usize + size - 1;
                let j0 = x as usize - 1;
                let j1 = x as usize + size - 1;
                let score = totals[w * i1 + j1] - totals[w * i1 + j0] - totals[w * i0 + j1]
                    + totals[w * i0 + j0];
                if score > best_score {
                    best_score = score;
                    best_square = Square {
                        x: x as i32,
                        y: y as i32,
                        size,
                    };
                }
            }
        }
    }

    (best_square, best_score)
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
        assert_eq!(
            solve(18, 300, 3..4),
            (
                Square {
                    x: 33,
                    y: 45,
                    size: 3
                },
                29
            )
        );
        assert_eq!(
            solve(42, 300, 3..4),
            (
                Square {
                    x: 21,
                    y: 61,
                    size: 3
                },
                30
            )
        );
    }

    #[test]
    fn part1() {
        assert_eq!(
            solve(7400, 300, 3..4),
            (
                Square {
                    x: 34,
                    y: 72,
                    size: 3
                },
                29
            )
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            solve(7400, 300, 1..301),
            (
                Square {
                    x: 233,
                    y: 187,
                    size: 13
                },
                91
            )
        );
    }
}
