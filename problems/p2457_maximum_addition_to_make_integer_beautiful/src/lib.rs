pub struct Solution {}

impl Solution {
    pub fn make_integer_beautiful(n: i64, target: i32) -> i64 {
        let mut remains = n;
        let mut digits = Vec::new();
        while 0 < remains {
            digits.push(remains % 10);
            remains /= 10;
        }
        digits.push(0);

        let mut sum = digits.iter().sum::<i64>();
        let target = target as i64;
        let mut carry = 0;
        let mut pow = 1;
        let mut result = 0;

        for i in 0..digits.len() {
            if carry == 1 {
                if digits[i] == 9 {
                    sum -= 9;
                    pow *= 10;
                    continue;
                } else {
                    digits[i] += 1;
                    sum += 1;
                    carry = 0;
                }
            }

            if sum <= target {
                break;
            }

            sum -= digits[i];
            if 0 < digits[i] {
                result += (10 - digits[i]) * pow;
                carry = 1;
            }
            pow *= 10;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2457() {
        assert_eq!(Solution::make_integer_beautiful(590, 1), 410);
        assert_eq!(Solution::make_integer_beautiful(19, 1), 81);
        assert_eq!(Solution::make_integer_beautiful(16, 6), 4);
        assert_eq!(Solution::make_integer_beautiful(467, 6), 33);
        assert_eq!(Solution::make_integer_beautiful(1, 1), 0);
    }
}
