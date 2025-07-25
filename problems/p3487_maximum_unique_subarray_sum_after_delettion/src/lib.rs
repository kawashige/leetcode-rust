pub struct Solution {}

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        if nums.iter().all(|n| n < &0) {
            return nums.into_iter().max().unwrap();
        }
        let mut result = 0;
        let mut seen = vec![false; 101];
        for n in nums {
            if 0 < n && !seen[n as usize] {
                result += n;
                seen[n as usize] = true;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3487() {
        assert_eq!(Solution::max_sum(vec![1, 2, 3, 4, 5]), 15);
        assert_eq!(Solution::max_sum(vec![1, 1, 0, 1, 1]), 1);
        assert_eq!(Solution::max_sum(vec![1, 2, -1, -2, 1, 0, -1]), 3);
    }
}
