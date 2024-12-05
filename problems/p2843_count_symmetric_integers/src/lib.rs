pub struct Solution {}

impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        (low..=high)
            .filter(|i| {
                let mut i = *i;
                let mut digits = Vec::new();
                while 0 < i {
                    digits.push(i % 10);
                    i /= 10;
                }
                digits.len() % 2 == 0
                    && digits[..digits.len() / 2].iter().sum::<i32>()
                        == digits[digits.len() / 2..].iter().sum::<i32>()
            })
            .count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2843() {
        assert_eq!(Solution::count_symmetric_integers(1, 100), 9);
        assert_eq!(Solution::count_symmetric_integers(1200, 1230), 4);
    }
}
