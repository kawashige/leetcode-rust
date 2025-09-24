pub struct Solution {}

impl Solution {
    pub fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    pub fn lcm(a: i32, b: i32) -> i32 {
        a / Self::gcd(a, b) * b
    }

    pub fn max_length(nums: Vec<i32>) -> i32 {
        let mut max_len = 0;
        for i in 0..nums.len() {
            let mut product = nums[i];
            let mut len = 1;
            let mut lcm = nums[i];
            let mut gcd = nums[i];
            for j in (0..i).rev() {
                product *= nums[j];
                lcm = Self::lcm(lcm, nums[j]);
                gcd = Self::gcd(gcd, nums[j]);
                if product == lcm * gcd {
                    len = i - j + 1;
                }
            }
            max_len = max_len.max(len);
        }
        max_len as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3411() {
        assert_eq!(Solution::max_length(vec![1, 2, 1, 2, 1, 1, 1]), 5);
        assert_eq!(Solution::max_length(vec![2, 3, 4, 5, 6]), 3);
        assert_eq!(Solution::max_length(vec![1, 2, 3, 1, 4, 5, 1]), 5);
    }
}
