pub struct Solution {}

impl Solution {
    pub fn kth_palindrome(queries: Vec<i32>, int_length: i32) -> Vec<i64> {
        queries
            .into_iter()
            .map(|q| {
                let first_half = (10_i32.pow((int_length as u32 + 1) / 2 - 1) + q - 1).to_string();
                if int_length < first_half.len() as i32 * 2 - int_length % 2 {
                    return -1;
                }
                format!(
                    "{}{}",
                    first_half,
                    first_half
                        .chars()
                        .rev()
                        .skip(int_length as usize % 2)
                        .collect::<String>()
                )
                .parse::<i64>()
                .unwrap()
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2217() {
        assert_eq!(
            Solution::kth_palindrome(vec![1, 2, 3, 4, 5, 90], 3),
            vec![101, 111, 121, 131, 141, 999]
        );
        assert_eq!(
            Solution::kth_palindrome(vec![2, 4, 6], 4),
            vec![1111, 1331, 1551]
        );
    }
}
