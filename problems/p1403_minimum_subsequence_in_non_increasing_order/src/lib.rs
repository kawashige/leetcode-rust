pub struct Solution {}

impl Solution {
    pub fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let sum = nums.iter().sum::<i32>();
        let mut subsequence_sum = 0;
        let mut result = Vec::new();
        for x in nums.into_iter().rev() {
            if subsequence_sum > sum - subsequence_sum {
                break;
            }
            result.push(x);
            subsequence_sum += x;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1403() {
        assert_eq!(Solution::min_subsequence(vec![4, 3, 10, 9, 8]), vec![10, 9]);
        assert_eq!(
            Solution::min_subsequence(vec![4, 4, 7, 6, 7]),
            vec![7, 7, 6]
        );
        assert_eq!(Solution::min_subsequence(vec![6]), vec![6]);
    }
}
