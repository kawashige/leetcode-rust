pub struct Solution {}

impl Solution {
    pub fn largest_integer(n: i32, s: i32) -> i32 {
        if n * 9 < s {
            return -1;
        }

        let mut result = 0;
        let mut s = s;
        for _ in 0..n {
            result = result * 10 + s.min(9);
            s -= s.min(9);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_4000() {
        assert_eq!(Solution::largest_integer(2, 9), 90);
        assert_eq!(Solution::largest_integer(2, 19), -1);
        assert_eq!(Solution::largest_integer(5, 0), 0);
    }
}

fn main() {}
