pub struct Solution {}

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut max = if nums.len() == 0 { 0 } else { 1 };
        let mut s = 0;
        for i in 1..nums.len() {
            if nums[i - 1] < nums[i] {
                continue;
            }
            max = std::cmp::max(max, i - s);
            s = i;
        }
        std::cmp::max(max, nums.len() - s) as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0674() {
        assert_eq!(Solution::find_length_of_lcis(vec![]), 0);
        assert_eq!(Solution::find_length_of_lcis(vec![1]), 1);
        assert_eq!(Solution::find_length_of_lcis(vec![1, 3, 4, 7]), 4);
        assert_eq!(Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]), 3);
        assert_eq!(Solution::find_length_of_lcis(vec![2, 2, 2, 2, 2]), 1);
    }
}
