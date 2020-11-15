pub struct Solution {}

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        for i in 1..=num {
            let n = i.overflowing_mul(i);
            if n.1 {
                return false;
            } else if num == n.0 {
                return true;
            } else if num < n.0 {
                break;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0367() {
        assert!(!Solution::is_perfect_square(2147483647));
        assert!(Solution::is_perfect_square(1));
        assert!(Solution::is_perfect_square(16));
        assert!(!Solution::is_perfect_square(14));
    }
}
