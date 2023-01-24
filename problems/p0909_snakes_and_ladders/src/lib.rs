use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let mut ladders = vec![-1; board.len() * board.len() + 1];

        let mut idx = 1;
        for i in (0..board.len()).rev() {
            if (board.len() - i) % 2 == 1 {
                for j in 0..board[0].len() {
                    ladders[idx] = board[i][j];
                    idx += 1;
                }
            } else {
                for j in (0..board[0].len()).rev() {
                    ladders[idx] = board[i][j];
                    idx += 1;
                }
            }
        }

        let mut deque = VecDeque::new();
        deque.push_back((1, 0));
        let mut seen = vec![false; ladders.len()];

        while let Some((i, c)) = deque.pop_front() {
            if seen[i] {
                continue;
            }

            if i == ladders.len() - 1 {
                return c;
            }

            seen[i] = true;

            for j in 1..7 {
                if i + j >= ladders.len() {
                    continue;
                }

                deque.push_back((
                    if ladders[i + j] != -1 {
                        ladders[i + j] as usize
                    } else {
                        i + j
                    },
                    c + 1,
                ));
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0909() {
        assert_eq!(
            Solution::snakes_and_ladders(vec![
                vec![-1, -1, -1, 30, -1, 144, 124, 135, -1, -1, -1, -1, -1],
                vec![-1, -1, -1, -1, -1, -1, 167, 93, -1, -1, -1, -1, -1],
                vec![-1, -1, -1, -1, -1, -1, -1, 77, -1, -1, 90, -1, -1],
                vec![-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
                vec![-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 122, -1],
                vec![-1, -1, -1, 23, -1, -1, -1, -1, -1, 155, -1, -1, -1],
                vec![-1, -1, 140, 29, -1, -1, -1, -1, -1, -1, -1, -1, -1],
                vec![-1, -1, -1, -1, -1, 115, -1, -1, -1, 109, -1, -1, 5],
                vec![-1, 57, -1, 99, 121, -1, -1, 132, -1, -1, -1, -1, -1],
                vec![-1, 48, -1, -1, -1, 68, -1, -1, -1, -1, 31, -1, -1],
                vec![-1, 163, 147, -1, 77, -1, -1, 114, -1, -1, 80, -1, -1],
                vec![-1, -1, -1, -1, -1, 57, 28, -1, -1, 129, -1, -1, -1],
                vec![-1, -1, -1, -1, -1, -1, -1, -1, -1, 87, -1, -1, -1]
            ]),
            6
        );
        assert_eq!(
            Solution::snakes_and_ladders(vec![
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, 35, -1, -1, 13, -1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![-1, 15, -1, -1, -1, -1]
            ]),
            4
        );
        assert_eq!(
            Solution::snakes_and_ladders(vec![vec![-1, -1], vec![-1, 3]]),
            1
        );
        assert_eq!(
            Solution::snakes_and_ladders(vec![vec![1, 1, -1], vec![1, 1, 1], vec![-1, 1, 1]]),
            -1
        );
        assert_eq!(
            Solution::snakes_and_ladders(vec![
                vec![-1, 1, 1, 1],
                vec![-1, 7, 1, 1],
                vec![1, 1, 1, 1],
                vec![-1, 1, 9, 1]
            ]),
            -1
        );
    }
}
