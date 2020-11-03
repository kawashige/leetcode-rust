pub struct Solution {}

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n < 1 {
            false
        } else {
            n == (3 as i32).pow((n as f64).log(3.0).round() as u32)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0326() {
        assert!(Solution::is_power_of_three(243));
        assert!(Solution::is_power_of_three(27));
        assert!(!Solution::is_power_of_three(0));
        assert!(Solution::is_power_of_three(1));
        assert!(Solution::is_power_of_three(9));
        assert!(!Solution::is_power_of_three(45));
    }
}
