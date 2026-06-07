pub struct Solution {}

impl Solution {
    pub fn remove_zeros(n: i64) -> i64 {
        let mut n = n;
        let mut result = 0;
        let mut pow = 1;

        while 0 < n {
            if n % 10 != 0 {
                result = result + pow * (n % 10);
                pow *= 10
            }
            n /= 10
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3276() {
        assert_eq!(Solution::remove_zeros(1020030), 123);
        assert_eq!(Solution::remove_zeros(1), 1);
    }
}

fn main() {}
