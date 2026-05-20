pub struct Solution {}

impl Solution {
    pub fn decimal_representation(n: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let mut n = n;
        let mut pow = 1;
        while 0 < n {
            let x = pow * (n % 10);
            if 0 < x {
                result.push(x);
            }
            pow *= 10;
            n /= 10;
        }

        result.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3697() {
        assert_eq!(Solution::decimal_representation(537), [500, 30, 7]);
        assert_eq!(Solution::decimal_representation(102), [100, 2]);
        assert_eq!(Solution::decimal_representation(6), [6]);
    }
}

fn main() {}
