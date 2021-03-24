use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn advantage_count(mut a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        a.sort_unstable();
        let mut deque = VecDeque::from(a);

        let mut b = b.into_iter().enumerate().collect::<Vec<_>>();
        b.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        let mut result = b
            .into_iter()
            .map(|(i, x)| {
                (
                    i,
                    if x < deque[deque.len() - 1] {
                        deque.pop_back().unwrap()
                    } else {
                        deque.pop_front().unwrap()
                    },
                )
            })
            .collect::<Vec<_>>();

        result.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        result.into_iter().map(|(_, x)| x).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day24() {
        assert_eq!(
            Solution::advantage_count(vec![2, 7, 11, 15], vec![1, 10, 4, 11]),
            vec![2, 11, 7, 15]
        );
        assert_eq!(
            Solution::advantage_count(vec![12, 24, 8, 32], vec![13, 25, 32, 11]),
            vec![24, 32, 8, 12]
        );
    }
}
