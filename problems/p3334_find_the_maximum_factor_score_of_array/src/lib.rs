pub struct Solution {}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i64 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0] as i64 * nums[0] as i64;
        }

        let mut l = nums[0] as i64;
        let mut g = nums[0] as i64;
        for i in 0..nums.len() {
            l = lcm(l, nums[i] as i64);
            g = gcd(g, nums[i] as i64);
        }
        let mut result = l * g;

        for i in 0..nums.len() {
            let mut l = nums[if i == 0 { 1 } else { 0 }] as i64;
            let mut g = nums[if i == 0 { 1 } else { 0 }] as i64;
            for j in 0..nums.len() {
                if i == j {
                    continue;
                }
                l = lcm(l, nums[j] as i64);
                g = gcd(g, nums[j] as i64);
            }
            result = result.max(l * g)
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3334() {
        assert_eq!(Solution::max_score(vec![2, 4, 8, 16]), 64);
        assert_eq!(Solution::max_score(vec![1, 2, 3, 4, 5]), 60);
        assert_eq!(Solution::max_score(vec![3]), 9);
    }
}
