pub struct Solution {}

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .map(|i| {
                if i % 15 == 0 {
                    "FizzBuzz".to_string()
                } else if i % 5 == 0 {
                    "Buzz".to_string()
                } else if i % 3 == 0 {
                    "Fizz".to_string()
                } else {
                    i.to_string()
                }
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day26() {
        assert_eq!(
            vec![
                "1".to_string(),
                "2".to_string(),
                "Fizz".to_string(),
                "4".to_string(),
                "Buzz".to_string(),
                "Fizz".to_string(),
                "7".to_string(),
                "8".to_string(),
                "Fizz".to_string(),
                "Buzz".to_string(),
                "11".to_string(),
                "Fizz".to_string(),
                "13".to_string(),
                "14".to_string(),
                "FizzBuzz".to_string()
            ],
            Solution::fizz_buzz(15)
        );
    }
}
