pub struct Solution {}

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = vec![0; 32];
        let mut num = 0;
        let mut l = 0;
        let mut result = std::i32::MAX;

        for i in 0..nums.len() {
            for j in 0..32 {
                if nums[i] & 1 << j != 0 {
                    count[j] += 1;
                    if count[j] == 1 {
                        num += 2_i32.pow(j as u32);
                    }
                }
            }
            while l < i {
                let mut tmp = 0;
                for j in 0..32 {
                    if nums[l] & 1 << j != 0 {
                        if count[j] == 1 {
                            tmp += 2_i32.pow(j as u32);
                        }
                    }
                }
                if k <= num - tmp {
                    for j in 0..32 {
                        if nums[l] & 1 << j != 0 {
                            count[j] -= 1;
                        }
                    }
                    num -= tmp;
                    l += 1;
                } else {
                    break;
                }
            }
            if k <= num {
                result = result.min((i - l) as i32 + 1);
            }
        }

        if result == std::i32::MAX {
            -1
        } else {
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3097() {
        assert_eq!(Solution::minimum_subarray_length(vec![1, 2, 3], 2), 1);
        assert_eq!(Solution::minimum_subarray_length(vec![2, 1, 8], 10), 3);
        assert_eq!(Solution::minimum_subarray_length(vec![1, 2], 0), 1);
    }
}
