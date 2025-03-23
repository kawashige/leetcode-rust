pub struct Solution {}

impl Solution {
    pub fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
        let nums = nums
            .windows(2)
            .map(|w| {
                if w[0] < w[1] {
                    1
                } else if w[0] == w[1] {
                    0
                } else {
                    -1
                }
            })
            .collect::<Vec<_>>();

        (0..nums.len() + 1 - pattern.len())
            .filter(|i| (0..pattern.len()).all(|j| nums[i + j] == pattern[j]))
            .count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3034() {
        assert_eq!(
            Solution::count_matching_subarrays(vec![1, 2, 3, 4, 5, 6], vec![1, 1]),
            4
        );
        assert_eq!(
            Solution::count_matching_subarrays(vec![1, 4, 4, 1, 3, 5, 5, 3], vec![1, 0, -1]),
            2
        );
    }
}
