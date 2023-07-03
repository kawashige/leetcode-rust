use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut max = 0;

        for i in 1..2_usize.pow(nums.len() as u32) {
            let mut bitwise_or = 0;
            for j in 0..nums.len() {
                if i & 1 << j != 0 {
                    bitwise_or |= nums[j];
                }
            }
            *map.entry(bitwise_or).or_insert(0) += 1;
            max = max.max(bitwise_or);
        }

        map[&max]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2044() {
        assert_eq!(Solution::count_max_or_subsets(vec![3, 1]), 2);
        assert_eq!(Solution::count_max_or_subsets(vec![2, 2, 2]), 7);
        assert_eq!(Solution::count_max_or_subsets(vec![3, 2, 1, 5]), 6);
    }
}
