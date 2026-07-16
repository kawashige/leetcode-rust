pub struct Solution {}

impl Solution {
    pub fn gcd(m: i32, n: i32) -> i32 {
        if m == 0 {
            return n;
        } else {
            Self::gcd(n % m, m)
        }
    }

    pub fn gcd_sum(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut mx = 0;
        let mut prefix_gcd = Vec::with_capacity(n);

        for i in 0..n {
            mx = mx.max(nums[i]);
            prefix_gcd.push(Self::gcd(mx, nums[i]));
        }
        prefix_gcd.sort_unstable();

        let mut result = 0;
        for i in 0..n / 2 {
            result += Self::gcd(prefix_gcd[i], prefix_gcd[n - 1 - i]) as i64;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3668() {
        assert_eq!(Solution::gcd_sum(vec![2, 6, 4]), 2);
        assert_eq!(Solution::gcd_sum(vec![3, 6, 2, 8]), 5);
    }
}

fn main() {}
