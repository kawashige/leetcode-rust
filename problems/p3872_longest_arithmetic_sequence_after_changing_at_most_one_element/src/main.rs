pub struct Solution {}

impl Solution {
    pub fn longest_arithmetic(nums: Vec<i32>) -> i32 {
        let mut l = vec![1; nums.len()];
        l[1] = 2;
        for i in 2..nums.len() {
            l[i] = if nums[i] - nums[i - 1] == nums[i - 1] - nums[i - 2] {
                l[i - 1] + 1
            } else {
                2
            };
        }

        let mut r = vec![1; nums.len()];
        r[nums.len() - 2] = 2;
        for i in (0..nums.len() - 2).rev() {
            r[i] = if nums[i + 1] - nums[i] == nums[i + 2] - nums[i + 1] {
                r[i + 1] + 1
            } else {
                2
            };
        }

        let mut result = 0;
        for i in 0..nums.len() {
            if 0 < i {
                result = result.max(l[i - 1] + 1);
            }
            if i < nums.len() - 1 {
                result = result.max(r[i + 1] + 1);
            }
            if 0 < i && i < nums.len() - 1 {
                if (nums[i + 1] - nums[i - 1]) % 2 != 0 {
                    continue;
                }
                let d = (nums[i + 1] - nums[i - 1]) / 2;
                if (i - 1 == 0 || nums[i - 1] - nums[i - 2] == d)
                    && (i + 1 == nums.len() - 1 || nums[i + 2] - nums[i + 1] == d)
                {
                    result = result.max(l[i - 1] + r[i + 1] + 1);
                } else if i - 1 == 0 || nums[i - 1] - nums[i - 2] == d {
                    result = result.max(l[i - 1] + 2);
                } else if i + 1 == nums.len() - 1 || nums[i + 2] - nums[i + 1] == d {
                    result = result.max(r[i + 1] + 2);
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
    fn test_3872() {
        assert_eq!(
            Solution::longest_arithmetic(vec![79734, 13414, 52866, 11223, 46264, 42963]),
            4
        );
        assert_eq!(Solution::longest_arithmetic(vec![2, 2, 1, 1]), 3);
        assert_eq!(Solution::longest_arithmetic(vec![9, 7, 5, 10, 1]), 5);
        assert_eq!(Solution::longest_arithmetic(vec![1, 2, 6, 7]), 3);
    }
}

fn main() {}
