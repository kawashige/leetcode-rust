pub struct Solution {}

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n < 1 {
            return false;
        }

        for i in 0..32 {
            let num = 1 << i;
            if num < n {
                continue;
            }
            return num == n;
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0230() {
        assert!(!Solution::is_power_of_two(0));
        assert!(!Solution::is_power_of_two(-4));
        assert!(Solution::is_power_of_two(1));
        assert!(Solution::is_power_of_two(16));
        assert!(!Solution::is_power_of_two(3));
        assert!(Solution::is_power_of_two(4));
        assert!(!Solution::is_power_of_two(5));
    }
}
