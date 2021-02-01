pub struct Solution {}

impl Solution {
    pub fn gcd(m: i32, n: i32) -> i32 {
        if m == 0 {
            return n;
        } else {
            Self::gcd(n % m, m)
        }
    }

    pub fn fraction_addition(expression: String) -> String {
        let mut v = Vec::new();
        let mut numerator = 0;
        let mut num = 0;
        let mut sign: i32 = 1;

        for c in expression.chars() {
            match c {
                '-' | '+' => {
                    if num != 0 {
                        v.push((numerator * sign, num));
                        num = 0;
                    }
                    sign = if c == '-' { -1 } else { 1 };
                }
                '/' => {
                    numerator = num;
                    num = 0;
                }
                _ => num = num * 10 + c.to_digit(10).unwrap() as i32,
            }
        }
        v.push((numerator * sign, num));
        v.sort_unstable_by_key(|x| x.1);

        let (mut numerator, mut denominator) = v[0];
        for (next_numerator, next_denominator) in v[1..].iter() {
            if &denominator == next_denominator {
                numerator += next_numerator;
            } else {
                numerator = numerator * next_denominator + denominator * next_numerator;
                denominator *= next_denominator;
            }
            if numerator == 0 {
                denominator = 1;
            } else {
                let gcd = Self::gcd(numerator.abs(), denominator.abs());
                numerator /= gcd;
                denominator /= gcd;
            }
        }

        format!("{}/{}", numerator, denominator)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0592() {
        assert_eq!(
            Solution::fraction_addition("-1/2+1/2".to_string()),
            "0/1".to_string()
        );
        assert_eq!(
            Solution::fraction_addition("-1/2+1/2+1/3".to_string()),
            "1/3".to_string()
        );
        assert_eq!(
            Solution::fraction_addition("1/3-1/2".to_string()),
            "-1/6".to_string()
        );
        assert_eq!(
            Solution::fraction_addition("5/3+1/3".to_string()),
            "2/1".to_string()
        );
    }
}
