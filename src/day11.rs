use std::ops::Range;

fn power_level(x: i32, y: i32, serial_number: i32) -> i32 {
    let id = x + 10;
    let power = (id * y + serial_number) * id;
    (power % 1_000) / 100 - 5
}

#[derive(Debug, Eq, PartialEq)]
struct Square(usize, usize, usize); // x, y, size

struct SummedAreaTable {
    width: usize,
    height: usize,
    table: Vec<i32>,
}

impl SummedAreaTable {
    fn new(width: usize, height: usize, values: Vec<i32>) -> SummedAreaTable {
        assert_eq!(values.len(), width * height);
        // table(i, j) = Sum { values(i', j') | i' <- 1..i, j' <- 1..j }
        let mut table = vec![0; (width + 1) * (height + 1)];
        for i in 1..=height {
            for j in 1..=width {
                table[(width + 1) * i + j] = table[(width + 1) * (i - 1) + j]
                    + table[(width + 1) * i + (j - 1)]
                    - table[(width + 1) * (i - 1) + (j - 1)]
                    + values[width * (i - 1) + (j - 1)];
            }
        }
        SummedAreaTable {
            width,
            height,
            table,
        }
    }

    fn get(&self, xs: Range<usize>, ys: Range<usize>) -> i32 {
        self.table[(self.width + 1) * (xs.end - 1) + (ys.end - 1)]
            - self.table[(self.width + 1) * (xs.end - 1) + (ys.start - 1)]
            - self.table[(self.width + 1) * (xs.start - 1) + (ys.end - 1)]
            + self.table[(self.width + 1) * (xs.start - 1) + (ys.start - 1)]
    }
}

fn solve(serial_number: i32, grid_size: usize, sizes: Range<usize>) -> (Square, i32) {
    let mut grid = vec![0; grid_size * grid_size];
    for y in 1..=grid_size {
        for x in 1..=grid_size {
            grid[grid_size * (y - 1) + (x - 1)] = power_level(y as i32, x as i32, serial_number);
        }
    }

    let table = SummedAreaTable::new(grid_size, grid_size, grid);

    let mut best_score = 0;
    let mut best_square = Square(0, 0, 0);
    for size in sizes {
        for y in 1..=grid_size + 1 - size {
            for x in 1..=grid_size + 1 - size {
                let score = table.get(x..x + size, y..y + size);
                if score > best_score {
                    best_score = score;
                    best_square = Square(x, y, size);
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
        assert_eq!(solve(18, 300, 3..4), (Square(33, 45, 3), 29));
        assert_eq!(solve(42, 300, 3..4), (Square(21, 61, 3), 30));
    }

    #[test]
    fn part1() {
        assert_eq!(solve(7400, 300, 3..4), (Square(34, 72, 3), 29));
    }

    #[test]
    fn part2() {
        assert_eq!(solve(7400, 300, 1..301), (Square(233, 187, 13), 91));
    }
}
