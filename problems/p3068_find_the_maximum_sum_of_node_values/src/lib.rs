pub struct Solution {}

impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut sum = 0;
        let mut xor_diff = Vec::with_capacity(nums.len());

        for i in 0..nums.len() {
            sum += nums[i] as i64;
            xor_diff.push((nums[i] ^ k) - nums[i]);
        }

        xor_diff.sort_unstable_by(|a, b| b.cmp(&a));
        let mut i = 0;
        while i + 1 < xor_diff.len() {
            if 0 <= xor_diff[i] {
                sum += 0.max(xor_diff[i] + xor_diff[i + 1]) as i64;
            } else {
                break;
            }
            i += 2;
        }

        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3068() {
        assert_eq!(
            Solution::maximum_value_sum(
                vec![24, 78, 1, 97, 44],
                6,
                vec![vec![0, 2], vec![1, 2], vec![4, 2], vec![3, 4]]
            ),
            260
        );
        assert_eq!(
            Solution::maximum_value_sum(vec![1, 2, 1], 3, vec![vec![0, 1], vec![0, 2]]),
            6
        );
        assert_eq!(
            Solution::maximum_value_sum(vec![2, 3], 7, vec![vec![0, 1]]),
            9
        );
        assert_eq!(
            Solution::maximum_value_sum(
                vec![7, 7, 7, 7, 7, 7],
                3,
                vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5]]
            ),
            42
        );
    }
}
