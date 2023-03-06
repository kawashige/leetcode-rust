pub struct Solution {}

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut counts = vec![[0; 2]; nums.len()];

        let mut result = 0;
        let mut bound = nums.len();
        let mut j = nums.len();

        for i in 0..nums.len() {
            if (min_k..=max_k).contains(&nums[i]) {
                if bound == nums.len() {
                    bound = i;
                    j = i;
                }
                if 0 < i {
                    counts[i][0] += counts[i - 1][0];
                    counts[i][1] += counts[i - 1][1];
                }
                if nums[i] == min_k {
                    counts[i][0] += 1;
                }
                if nums[i] == max_k {
                    counts[i][1] += 1;
                }

                if 0 < counts[i][0] && 0 < counts[i][1] {
                    while j < i && (0..2).all(|k| 0 < counts[i][k] - counts[j][k]) {
                        j += 1;
                    }
                    result += (j - bound + 1) as i64;
                }
            } else {
                bound = nums.len();
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2444() {
        assert_eq!(Solution::count_subarrays(vec![1, 3, 5, 2, 7, 5], 1, 5), 2);
        assert_eq!(Solution::count_subarrays(vec![1, 1, 1, 1], 1, 1), 10);
    }
}
