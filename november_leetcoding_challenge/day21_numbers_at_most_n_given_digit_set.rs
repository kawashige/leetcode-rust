pub struct Solution {}

impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        if digits.is_empty() {
            return 0;
        }

        let n_chars = n
            .to_string()
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        let mut digits = digits;
        digits.sort();

        let n_l = n_chars.len() as i32;
        let d_l = digits.len() as i32;

        if n_l == 1 {
            return digits.into_iter().filter(|d| d <= &n_chars[0]).count() as i32;
        }
        if d_l == 1 {
            return n_l - 1 + if digits[0] <= n_chars[0] { 1 } else { 0 };
        }

        let mut result = (d_l * (1 - d_l.pow(n_l as u32 - 1))) / (1 - d_l);
        println!("result 1: {}", result);
        let count = digits.iter().filter(|d| *d < &n_chars[0]).count() as i32;
        println!("count: {}", count);
        result += count * d_l.pow(n_l as u32 - 1 as u32);
        println!("result 2: {}", result);

        if count < d_l && digits[count as usize] == n_chars[0] {
            for i in 1..n_chars.len() {
                if i == n_chars.len() - 1 {
                    result += digits.iter().filter(|d| *d <= &n_chars[i]).count() as i32;
                    println!("result last: {}", result);
                } else {
                    let count = digits.iter().filter(|d| *d < &n_chars[i]).count() as i32;
                    result += count * d_l.pow(n_l as u32 - 1 - i as u32);
                    println!("result : {}, count: {}", result, count);

                    if count > d_l - 1 || digits[count as usize] != n_chars[i] {
                        break;
                    }
                }
            }
        }

        println!("result 3: {}", result);
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day21() {
        assert_eq!(
            363,
            Solution::at_most_n_given_digit_set(
                vec!["2".to_string(), "3".to_string(), "6".to_string()],
                71066
            )
        );
        assert_eq!(
            5760,
            Solution::at_most_n_given_digit_set(
                vec![
                    "1".to_string(),
                    "2".to_string(),
                    "3".to_string(),
                    "4".to_string(),
                    "5".to_string(),
                    "6".to_string(),
                    "7".to_string(),
                    "8".to_string(),
                    "9".to_string(),
                ],
                7799
            )
        );
        assert_eq!(
            171,
            Solution::at_most_n_given_digit_set(
                vec![
                    "1".to_string(),
                    "2".to_string(),
                    "3".to_string(),
                    "4".to_string(),
                    "6".to_string(),
                    "7".to_string(),
                    "9".to_string(),
                ],
                333
            )
        );
        assert_eq!(
            1,
            Solution::at_most_n_given_digit_set(vec!["55".to_string()], 9)
        );
        assert_eq!(
            3,
            Solution::at_most_n_given_digit_set(vec!["1".to_string()], 834)
        );
        assert_eq!(
            6,
            Solution::at_most_n_given_digit_set(
                vec!["5".to_string(), "7".to_string(), "8".to_string()],
                59
            )
        );
        assert_eq!(
            2,
            Solution::at_most_n_given_digit_set(vec!["5".to_string(), "6".to_string()], 19)
        );
        assert_eq!(
            29523,
            Solution::at_most_n_given_digit_set(
                vec!["1".to_string(), "4".to_string(), "9".to_string()],
                1000000000
            )
        );
        assert_eq!(
            20,
            Solution::at_most_n_given_digit_set(
                vec![
                    "1".to_string(),
                    "3".to_string(),
                    "5".to_string(),
                    "7".to_string()
                ],
                100
            )
        );
        assert_eq!(
            1,
            Solution::at_most_n_given_digit_set(vec!["7".to_string()], 8)
        );
        assert_eq!(
            2,
            Solution::at_most_n_given_digit_set(vec!["1".to_string()], 11)
        );
        assert_eq!(
            1,
            Solution::at_most_n_given_digit_set(vec!["3".to_string(), "5".to_string()], 4)
        );
    }
}
