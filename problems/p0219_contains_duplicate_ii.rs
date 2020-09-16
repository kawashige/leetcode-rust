pub struct Solution {}

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if k == 0 && nums.len() > 0 {
            return false;
        }
        let mut subset = Vec::with_capacity(k as usize);
        for i in 0..nums.len() {
            if subset.contains(&nums[i]) {
                return true;
            }
            if k <= i as i32 {
                subset.remove(0);
            }
            subset.push(nums[i]);
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0219() {
        assert!(!Solution::contains_nearby_duplicate(vec![], 1));
        assert!(!Solution::contains_nearby_duplicate(vec![2, 3, 3], 0));
        assert!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
        assert!(Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1));
        assert!(!Solution::contains_nearby_duplicate(
            vec![1, 2, 3, 1, 2, 3],
            2
        ));
    }
}
