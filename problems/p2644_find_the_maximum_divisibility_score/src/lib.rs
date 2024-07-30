pub struct Solution {}

impl Solution {
    pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        divisors
            .into_iter()
            .max_by_key(|d| (nums.iter().filter(|num| **num % *d == 0).count(), -1 * d))
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2644() {
        assert_eq!(
            Solution::max_div_score(vec![2, 9, 15, 50], vec![5, 3, 7, 2]),
            2
        );
        assert_eq!(
            Solution::max_div_score(vec![4, 7, 9, 3, 9], vec![5, 2, 3]),
            3
        );
        assert_eq!(
            Solution::max_div_score(vec![20, 14, 21, 10], vec![10, 16, 20]),
            10
        );
    }
}
