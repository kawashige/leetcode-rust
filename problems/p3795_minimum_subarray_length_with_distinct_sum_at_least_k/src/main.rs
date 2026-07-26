pub struct Solution {}

impl Solution {
    pub fn min_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut r = 0;
        let mut sum = nums[0];
        let mut freq = vec![0; 100_001];
        freq[nums[0] as usize] += 1;

        let mut result = std::usize::MAX;

        for l in 0..nums.len() {
            if 0 < l {
                if freq[nums[l - 1] as usize] == 1 {
                    sum -= nums[l - 1] as i32;
                }
                freq[nums[l - 1] as usize] -= 1;
            }
            while r + 1 < nums.len() && sum < k {
                r += 1;
                if freq[nums[r] as usize] == 0 {
                    sum += nums[r] as i32;
                }
                freq[nums[r] as usize] += 1;
            }

            if k <= sum {
                result = result.min(r - l + 1);
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3765() {
        assert_eq!(Solution::min_length(vec![2, 2, 3, 1], 4), 2);
        assert_eq!(Solution::min_length(vec![3, 2, 3, 4], 5), 2);
        assert_eq!(Solution::min_length(vec![5, 5, 4], 5), 1);
    }
}

fn main() {}
