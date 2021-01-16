pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut counts = [0; 2];
        let mut diff_index = HashMap::new();
        let mut max = 0;

        for (i, n) in nums.into_iter().enumerate() {
            counts[n as usize] += 1;
            if counts[0] == counts[1] {
                max = std::cmp::max(max, i + 1);
            } else {
                if let Some(j) = diff_index.get(&(counts[0] - counts[1])) {
                    max = std::cmp::max(max, i - j);
                } else {
                    diff_index.insert(counts[0] - counts[1], i);
                }
            }
        }

        max as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0525() {
        assert_eq!(2, Solution::find_max_length(vec![0, 1, 1]));
        assert_eq!(0, Solution::find_max_length(vec![]));
        assert_eq!(0, Solution::find_max_length(vec![1, 1, 1]));
        assert_eq!(2, Solution::find_max_length(vec![0, 1]));
        assert_eq!(2, Solution::find_max_length(vec![0, 1, 0]));
        assert_eq!(6, Solution::find_max_length(vec![0, 0, 0, 1, 1, 1, 1]));
        assert_eq!(6, Solution::find_max_length(vec![0, 0, 1, 0, 0, 1, 1, 0]));
    }
}
