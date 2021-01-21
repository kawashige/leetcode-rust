pub struct Solution {}

impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut result = Vec::with_capacity(k);

        let l = nums.len();
        for i in 0..l {
            while !result.is_empty()
                && k < result.len() + l - i
                && &nums[i] < result.last().unwrap()
            {
                result.pop();
            }
            if result.len() < k {
                result.push(nums[i]);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_() {
        assert_eq!(vec![1, 1], Solution::most_competitive(vec![1, 5, 2, 1], 2));
        assert_eq!(vec![2, 6], Solution::most_competitive(vec![3, 5, 2, 6], 2));
        assert_eq!(
            vec![2, 3, 3, 4],
            Solution::most_competitive(vec![2, 4, 3, 3, 5, 4, 9, 6], 4)
        );
    }
}
