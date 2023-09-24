pub struct Solution {}

impl Solution {
    pub fn smallest_number(num: i64) -> i64 {
        if num == 0 {
            return 0;
        }

        let sign = num.signum();
        let mut digits = [0; 10];

        let mut num = num.abs();
        while 0 < num {
            digits[(num % 10) as usize] += 1;
            num /= 10;
        }

        let mut result;
        if 0 <= sign {
            result = (1..10).find(|i| 0 < digits[*i as usize]).unwrap();
            digits[result as usize] -= 1;
            for i in 0..10 {
                for _ in 0..digits[i] {
                    result = result * 10 + i as i64;
                }
            }
        } else {
            result = (1..10).rev().find(|i| 0 < digits[*i as usize]).unwrap();
            digits[result as usize] -= 1;
            for i in (0..10).rev() {
                for _ in 0..digits[i] {
                    result = result * 10 + i as i64;
                }
            }
        }

        result * sign
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2165() {
        assert_eq!(Solution::smallest_number(310), 103);
        assert_eq!(Solution::smallest_number(-7605), -7650);
    }
}
