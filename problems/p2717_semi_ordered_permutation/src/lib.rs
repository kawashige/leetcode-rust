pub struct Solution {}

impl Solution {
    pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
        let first = (0..nums.len()).find(|i| nums[*i] == 1).unwrap();
        let last = (0..nums.len())
            .find(|i| nums[*i] == nums.len() as i32)
            .unwrap();

        (first + nums.len() - 1 - last - if last < first { 1 } else { 0 }) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2717() {
        assert_eq!(Solution::semi_ordered_permutation(vec![2, 1, 4, 3]), 2);
        assert_eq!(Solution::semi_ordered_permutation(vec![2, 4, 1, 3]), 3);
        assert_eq!(Solution::semi_ordered_permutation(vec![1, 3, 4, 2, 5]), 0);
    }
}
