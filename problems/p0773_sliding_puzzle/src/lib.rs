use std::collections::{HashSet, VecDeque};

pub struct Solution {}

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((board, 0));

        while let Some((b, m)) = queue.pop_front() {
            let key = b
                .iter()
                .flatten()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(",");
            if key == "1,2,3,4,5,0" {
                return m;
            }
            if seen.contains(&key) {
                continue;
            }
            seen.insert(key);

            let mut zero = (0, 0);
            for i in 0..2 {
                for j in 0..3 {
                    if b[i][j] == 0 {
                        zero = (i, j);
                    }
                }
            }

            for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
                let (new_i, new_j) = (zero.0 as i32 + di, zero.1 as i32 + dj);
                if !(0..2).contains(&new_i) || !(0..3).contains(&new_j) {
                    continue;
                }
                let mut new_b = b.clone();
                new_b[zero.0][zero.1] = new_b[new_i as usize][new_j as usize];
                new_b[new_i as usize][new_j as usize] = 0;
                queue.push_back((new_b, m + 1));
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0773() {
        assert_eq!(
            Solution::sliding_puzzle(vec![vec![1, 2, 3], vec![4, 0, 5]]),
            1
        );
        assert_eq!(
            Solution::sliding_puzzle(vec![vec![1, 2, 3], vec![5, 4, 0]]),
            -1
        );
        assert_eq!(
            Solution::sliding_puzzle(vec![vec![4, 1, 2], vec![5, 0, 3]]),
            5
        );
    }
}
