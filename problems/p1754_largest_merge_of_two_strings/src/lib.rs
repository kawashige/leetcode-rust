pub struct Solution {}

impl Solution {
    pub fn largest_merge(word1: String, word2: String) -> String {
        let mut merge = String::new();
        let mut i1 = 0;
        let mut i2 = 0;

        while i1 < word1.len() && i2 < word2.len() {
            if &word1[i1..] < &word2[i2..] {
                merge.push(word2.as_bytes()[i2] as char);
                i2 += 1
            } else {
                merge.push(word1.as_bytes()[i1] as char);
                i1 += 1
            }
        }

        merge
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1754() {
        assert_eq!(
            Solution::largest_merge("cabaa".to_string(), "bcaaa".to_string()),
            "cbcabaaaaa".to_string()
        );
        assert_eq!(
            Solution::largest_merge("abcabc".to_string(), "abdcaba".to_string()),
            "abdcabcabcaba".to_string()
        );
    }
}
