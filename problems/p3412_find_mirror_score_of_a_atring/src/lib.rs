pub struct Solution {}

impl Solution {
    pub fn calculate_score(s: String) -> i64 {
        let mut indices = vec![Vec::new(); 26];
        let mut result = 0;

        for i in 0..s.len() {
            let j = (s.as_bytes()[i] - b'a') as usize;
            if !indices[25 - j].is_empty() {
                result += (i - indices[25 - j].pop().unwrap()) as i64;
            } else {
                indices[j].push(i);
            }
        }

        result
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3412() {
        assert_eq!(Solution::calculate_score("aczzx".to_string()), 5);
        assert_eq!(Solution::calculate_score("abcdef".to_string()), 0);
    }
}
