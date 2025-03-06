pub struct Solution {}

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut result = 0;
        for i in 0..word.len() {
            result += i / 8 + 1;
        }
        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3014() {
        assert_eq!(Solution::minimum_pushes("abcde".to_string()), 5);
        assert_eq!(Solution::minimum_pushes("xycdefghij".to_string()), 12);
    }
}
