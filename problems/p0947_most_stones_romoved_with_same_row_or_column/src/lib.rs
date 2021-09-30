use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let mut seen = vec![false; stones.len()];
        let mut r = 0;

        for i in 0..stones.len() {
            if seen[i] {
                continue;
            }

            let mut deque = VecDeque::new();
            let mut count = 0;
            deque.push_back(i);

            while let Some(j) = deque.pop_front() {
                if seen[j] {
                    continue;
                }
                count += 1;
                seen[j] = true;
                for k in 0..stones.len() {
                    if !seen[k] && (stones[k][0] == stones[j][0] || stones[k][1] == stones[j][1]) {
                        deque.push_back(k);
                    }
                }
            }
            r += count - 1;
        }

        r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0947() {
        assert_eq!(
            Solution::remove_stones(vec![
                vec![0, 0],
                vec![0, 1],
                vec![1, 0],
                vec![1, 2],
                vec![2, 1],
                vec![2, 2]
            ]),
            5
        );
        assert_eq!(
            Solution::remove_stones(vec![
                vec![0, 0],
                vec![0, 2],
                vec![1, 1],
                vec![2, 0],
                vec![2, 2]
            ]),
            3
        );
    }
}
