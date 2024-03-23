pub struct Solution {}

impl Solution {
    pub fn gcd(m: i32, n: i32) -> i32 {
        if m == 0 {
            return n;
        } else {
            Self::gcd(n % m, m)
        }
    }

    pub fn subarray_gcd(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;

        for i in 0..nums.len() {
            let mut current_gcd = nums[i];
            if current_gcd == k {
                result += 1;
            }

            for j in (0..i).rev() {
                current_gcd = Self::gcd(current_gcd, nums[j]);
                if current_gcd < k {
                    break;
                }
                if current_gcd == k {
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
    fn test_2447() {
        assert_eq!(Solution::subarray_gcd(vec![9, 3, 1, 2, 6, 3], 3), 4);
        assert_eq!(Solution::subarray_gcd(vec![4], 7), 0);
    }
}
