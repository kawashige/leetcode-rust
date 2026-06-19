pub struct Solution {}

impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
        let mut acc = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            acc[i + 1] += acc[i];
            if nums[i] == target {
                acc[i + 1] += 1;
            }
        }

        let mut result = 0;

        for r in 0..nums.len() {
            for l in 0..=r {
                if r - l + 1 < 2 * (acc[r + 1] - acc[l]) {
                    result += 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3737() {
        assert_eq!(Solution::count_majority_subarrays(vec![1, 2, 2, 3], 2), 5);
        assert_eq!(Solution::count_majority_subarrays(vec![1, 1, 1, 1], 1), 10);
        assert_eq!(Solution::count_majority_subarrays(vec![1, 2, 3], 4), 0);
    }
}

fn main() {}
