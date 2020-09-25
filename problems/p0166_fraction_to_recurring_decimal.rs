pub struct Solution {}

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let denominator_abs = (denominator as i64).abs();
        let numerator_abs = (numerator as i64).abs();
        let integer_part = numerator_abs / denominator_abs;
        let mut remains = numerator_abs % denominator_abs;
        let sign = if denominator.signum() * numerator.signum() < 0 {
            "-"
        } else {
            ""
        };

        if remains == 0 {
            return format!("{}{}", sign, integer_part.to_string());
        }

        let mut decimal_part = Vec::new();
        let mut past_remains = vec![remains];
        while remains > 0 {
            remains *= 10;
            decimal_part.push(remains / denominator_abs);
            remains %= denominator_abs;
            if past_remains.contains(&remains) {
                break;
            }
            past_remains.push(remains);
        }

        let mut decimal_string = decimal_part
            .into_iter()
            .map(|d| d.to_string())
            .collect::<String>();

        if remains != 0 {
            let pos = past_remains.iter().position(|r| r == &remains).unwrap();
            decimal_string.insert(pos, '(');
            decimal_string.insert(decimal_string.len(), ')');
        }

        format!("{}{}.{}", sign, integer_part, decimal_string)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0166() {
        assert_eq!("0.5".to_string(), Solution::fraction_to_decimal(1, 2));
        assert_eq!("2".to_string(), Solution::fraction_to_decimal(2, 1));
        assert_eq!("0.(6)".to_string(), Solution::fraction_to_decimal(2, 3));
        assert_eq!("0.(012)".to_string(), Solution::fraction_to_decimal(4, 333));
        assert_eq!("0.2".to_string(), Solution::fraction_to_decimal(1, 5));
        assert_eq!("-0.(6)".to_string(), Solution::fraction_to_decimal(2, -3));
        assert_eq!("-0.(6)".to_string(), Solution::fraction_to_decimal(-2, 3));
        assert_eq!("0.(6)".to_string(), Solution::fraction_to_decimal(2, 3));
        assert_eq!("0".to_string(), Solution::fraction_to_decimal(0, 3));
        assert_eq!(
            "-2147483648".to_string(),
            Solution::fraction_to_decimal(-2147483648, 1)
        );
    }
}
