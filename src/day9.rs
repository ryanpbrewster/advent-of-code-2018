use std::collections::VecDeque;

fn play(num_players: usize, last_marble: usize) -> Vec<usize> {
    let mut ring = VecDeque::new();
    let mut scores = vec![0; num_players];

    ring.push_front(0);
    ring.push_front(1);
    for i in 2..=last_marble {
        if i % 23 == 0 {
            ring.rotate_right(7);
            scores[(i - 1) % num_players] += i + ring.pop_front().unwrap_or(0);
        } else {
            ring.rotate_left(2);
            ring.push_front(i);
        }
    }

    scores
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke() {
        assert_eq!(play(9, 25), vec![0, 0, 0, 0, 32, 0, 0, 0, 0]);
        assert_eq!(play(10, 1618).into_iter().max().unwrap(), 8317);
        assert_eq!(play(13, 7999).into_iter().max().unwrap(), 146373);
        assert_eq!(play(17, 1104).into_iter().max().unwrap(), 2764);
        assert_eq!(play(21, 6111).into_iter().max().unwrap(), 54718);
        assert_eq!(play(30, 5807).into_iter().max().unwrap(), 37305);
    }

    #[test]
    fn part1() {
        assert_eq!(play(411, 72059).into_iter().max().unwrap(), 429943);
    }

    #[test]
    fn part2() {
        assert_eq!(
            play(411, 72059 * 100).into_iter().max().unwrap(),
            3615691746
        );
    }
}
