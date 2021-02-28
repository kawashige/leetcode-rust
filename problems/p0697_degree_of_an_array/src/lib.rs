use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let mut max_count = 0;
        let mut min_length = 50001;
        for i in 0..nums.len() {
            let entry = count.entry(nums[i]).or_insert((0, i));
            entry.0 += 1;
            if max_count < entry.0 {
                max_count = entry.0;
                min_length = i - entry.1 + 1;
            } else if max_count == entry.0 {
                min_length = std::cmp::min(min_length, i - entry.1 + 1);
            }
        }
        min_length as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0697() {
        assert_eq!(Solution::find_shortest_sub_array(vec![1, 3, 2, 2, 3, 1]), 2);
        assert_eq!(Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1]), 2);
        assert_eq!(
            Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1, 4, 2]),
            6
        );
    }
}
