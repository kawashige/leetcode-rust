use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut map = HashMap::new();
        let mut max_len = 1;

        for i in 0..arr.len() {
            let len = map.get(&(arr[i] - difference)).unwrap_or(&0) + 1;
            if map.get(&arr[i]).unwrap_or(&0) < &len {
                map.insert(arr[i], len);
            }
            max_len = max_len.max(len);
        }

        max_len
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1219() {
        assert_eq!(Solution::longest_subsequence(vec![1, 2, 3, 4], 1), 4);
        assert_eq!(Solution::longest_subsequence(vec![1, 3, 5, 7], 1), 1);
        assert_eq!(
            Solution::longest_subsequence(vec![1, 5, 7, 8, 5, 3, 4, 2, 1], -2),
            4
        );
    }
}
