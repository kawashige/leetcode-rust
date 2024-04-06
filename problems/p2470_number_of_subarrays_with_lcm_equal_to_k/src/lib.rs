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

    pub fn subarray_lcm(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;

        for i in 0..nums.len() {
            let mut lcm = 1;
            for j in i..nums.len() {
                lcm = Self::lcm(lcm, nums[j]);
                if k < lcm {
                    break;
                } else if k == lcm {
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
    fn test_2470() {
        assert_eq!(Solution::subarray_lcm(vec![3, 6, 2, 7, 1], 6), 4);
        assert_eq!(Solution::subarray_lcm(vec![3], 2), 0);
    }
}
