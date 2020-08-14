pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        use std::collections::HashMap;
        let odd_count = s
            .chars()
            .fold(HashMap::new(), |mut acc, c| {
                let c = acc.entry(c).or_insert(0);
                *c += 1;
                acc
            })
            .values()
            .filter(|x| *x % 2 == 1)
            .count();
        (s.len() - odd_count.saturating_sub(1)) as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day14() {
        assert_eq!(7, Solution::longest_palindrome("abccccdd".to_string()));
        assert_eq!(4, Solution::longest_palindrome("ccdd".to_string()));
        assert_eq!(1, Solution::longest_palindrome("Aa".to_string()));
        assert_eq!(2, Solution::longest_palindrome("aa".to_string()));
    }
}
