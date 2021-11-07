pub struct Solution {}

impl Solution {
    pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, mut k: i32) -> i32 {
        nums.sort_unstable();

        for i in 0..nums.len() {
            if k == 0 || nums[i] >= 0 {
                break;
            }
            nums[i] *= -1;
            k -= 1;
        }

        if k > 0 && k % 2 == 1 {
            if let Some(i) = (0..nums.len()).min_by_key(|i| nums[*i]) {
                nums[i] *= -1;
            }
        }

        nums.iter().sum::<i32>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1005() {
        assert_eq!(Solution::largest_sum_after_k_negations(vec![4, 2, 3], 1), 5);
        assert_eq!(
            Solution::largest_sum_after_k_negations(vec![3, -1, 0, 2], 3),
            6
        );
        assert_eq!(
            Solution::largest_sum_after_k_negations(vec![2, -3, -1, 5, -4], 2),
            13
        );
    }
}
