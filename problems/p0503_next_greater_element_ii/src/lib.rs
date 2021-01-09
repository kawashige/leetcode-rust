pub struct Solution {}

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut max = std::i32::MIN;
        let mut left_max = Vec::new();
        for i in 0..nums.len() {
            max = std::cmp::max(max, nums[i]);
            left_max.push(max);
        }

        max = std::i32::MIN;
        let mut right_max = Vec::new();
        for i in (0..nums.len()).rev() {
            max = std::cmp::max(max, nums[i]);
            right_max.push(max);
        }
        right_max.reverse();

        let mut result = Vec::new();
        for i in 0..nums.len() {
            if i + 1 < nums.len() && nums[i] < right_max[i + 1] {
                result.push(*nums[(i + 1)..].iter().find(|n| &&nums[i] < n).unwrap());
            } else if 0 < i && nums[i] < left_max[i - 1] {
                result.push(*nums[..i].iter().find(|n| &&nums[i] < n).unwrap());
            } else {
                result.push(-1);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0503() {
        assert_eq!(
            vec![2, -1, 2],
            Solution::next_greater_elements(vec![1, 2, 1])
        );
        assert_eq!(vec![] as Vec<i32>, Solution::next_greater_elements(vec![]));
    }
}
