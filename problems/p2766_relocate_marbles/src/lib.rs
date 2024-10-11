use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn relocate_marbles(nums: Vec<i32>, move_from: Vec<i32>, move_to: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }
        for i in 0..move_from.len() {
            if move_from[i] == move_to[i] {
                continue;
            }
            let c = map[&move_from[i]];
            *map.entry(move_to[i]).or_insert(0) += c;
            map.remove(&move_from[i]);
        }

        let mut result = map.keys().cloned().into_iter().collect::<Vec<_>>();
        result.sort_unstable();
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2766() {
        assert_eq!(
            Solution::relocate_marbles(vec![1, 6, 7, 8], vec![1, 7, 2], vec![2, 9, 5]),
            vec![5, 6, 8, 9]
        );
        assert_eq!(
            Solution::relocate_marbles(vec![1, 1, 3, 3], vec![1, 3], vec![2, 2]),
            vec![2]
        );
    }
}
