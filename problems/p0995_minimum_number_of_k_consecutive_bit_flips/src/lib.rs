pub struct Solution {}

impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = vec![0; nums.len() + 1];
        let mut flips = 0;
        let k = k as usize;

        for i in 0..nums.len() {
            if 0 < i {
                count[i] += count[i - 1];
            }
            let num = (nums[i] + count[i]) % 2;
            if num == 0 {
                if i <= nums.len() - k {
                    flips += 1;
                    count[i] += 1;
                    count[i + k] -= 1;
                } else {
                    return -1;
                }
            }
        }

        flips
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0995() {
        assert_eq!(Solution::min_k_bit_flips(vec![0, 1, 0], 1), 2);
        assert_eq!(Solution::min_k_bit_flips(vec![1, 1, 0], 2), -1);
        assert_eq!(
            Solution::min_k_bit_flips(vec![0, 0, 0, 1, 0, 1, 1, 0], 3),
            3
        );
    }
}
