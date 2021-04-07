pub struct Solution {}

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        s.to_ascii_lowercase()
            .chars()
            .enumerate()
            .fold(0, |count, (i, c)| {
                if ['a', 'e', 'i', 'o', 'u'].contains(&c) {
                    count + if i < s.len() / 2 { 1 } else { -1 }
                } else {
                    count
                }
            })
            == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day07() {
        assert!(!Solution::halves_are_alike("bbbooo".to_string()));
        assert!(Solution::halves_are_alike("book".to_string()));
        assert!(!Solution::halves_are_alike("textbook".to_string()));
        assert!(!Solution::halves_are_alike("MerryChristmas".to_string()));
        assert!(Solution::halves_are_alike("AbCdEfGh".to_string()));
    }
}
