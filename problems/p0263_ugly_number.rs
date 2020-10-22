pub struct Solution {}

impl Solution {
    pub fn is_ugly(num: i32) -> bool {
        if num < 1 {
            false
        } else {
            let mut n = num;
            while n % 2 == 0 {
                n /= 2;
            }
            while n % 3 == 0 {
                n /= 3;
            }
            while n % 5 == 0 {
                n /= 5;
            }
            n == 1
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0263() {
        assert!(Solution::is_ugly(1));
        assert!(Solution::is_ugly(6));
        assert!(Solution::is_ugly(8));
        assert!(!Solution::is_ugly(14));
        assert!(!Solution::is_ugly(-4));
    }
}
