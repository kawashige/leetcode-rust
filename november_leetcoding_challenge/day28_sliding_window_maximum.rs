pub struct Solution {}

use std::collections::BTreeMap;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut results = Vec::new();
        let mut map: BTreeMap<i32, i32> = BTreeMap::new();

        let k = k as usize;
        for i in 0..nums.len() {
            if k <= i {
                *map.get_mut(&nums[i - k]).unwrap() -= 1;
                if map[&nums[i - k]] == 0 {
                    map.remove(&nums[i - k]);
                }
            }
            *map.entry(nums[i]).or_insert(0) += 1;
            if k - 1 <= i {
                results.push(*map.iter().rev().next().unwrap().0);
            }
        }

        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day28() {
        assert_eq!(
            vec![3, 3, 2, 5],
            Solution::max_sliding_window(vec![1, 3, 1, 2, 0, 5], 3)
        );
        assert_eq!(
            vec![2, 3, 4, 5, 6],
            Solution::max_sliding_window(vec![1, 2, 3, 4, 5, 6], 2)
        );
        assert_eq!(
            vec![3, 3, 5, 5, 6, 7],
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3)
        );
        assert_eq!(vec![1], Solution::max_sliding_window(vec![1], 1));
        assert_eq!(vec![1, -1], Solution::max_sliding_window(vec![1, -1], 1));
        assert_eq!(vec![11], Solution::max_sliding_window(vec![9, 11], 2));
        assert_eq!(vec![4], Solution::max_sliding_window(vec![4, -2], 2));
    }
}
