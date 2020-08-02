pub struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        for i in 0..nums.len() {
            let mut sum = 0;
            for j in i..nums.len() {
                sum += nums[j];
                if max < sum {
                    max = sum;
                }
            }
        }
        max
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_53() {
        assert_eq!(1, Solution::max_sub_array(vec![-2, 1]));
    }
}
