pub struct Solution {}

impl Solution {
    pub fn sum_and_multiply(n: i32) -> i64 {
        let mut sum = 0;
        let mut multiple = 0;
        let mut n = n as i64;
        let mut pow = 1;
        while 0 < n {
            if n % 10 != 0 {
                sum += n % 10;
                multiple += (n % 10) * pow;
                pow *= 10;
            }
            n /= 10;
        }

        sum * multiple
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3754() {
        assert_eq!(Solution::sum_and_multiply(10203004), 12340);
        assert_eq!(Solution::sum_and_multiply(1000), 1);
    }
}

fn main() {}
