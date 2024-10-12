pub struct Solution {}

impl Solution {
    pub fn recurse(s: &str, i: usize, candidates: &[String], count: i32) -> i32 {
        if i == s.len() {
            return count;
        }
        println!("i: {}, count: {}", i, count);
        for j in (0..candidates.len()).rev() {
            if s[i..].starts_with(&candidates[j]) {
                let result = Self::recurse(s, i + candidates[j].len(), candidates, count + 1);
                if result != -1 {
                    return result;
                }
            }
        }
        -1
    }

    pub fn minimum_beautiful_substrings(s: String) -> i32 {
        let mut candidates = Vec::new();
        let mut i = 1;
        while format!("{:b}", i).len() <= s.len() {
            candidates.push(format!("{:b}", i));
            i *= 5;
        }
        println!("{:?}", candidates);

        Self::recurse(&s, 0, &candidates, 0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2767() {
        assert_eq!(
            Solution::minimum_beautiful_substrings("1011".to_string()),
            2
        );
        assert_eq!(Solution::minimum_beautiful_substrings("111".to_string()), 3);
        assert_eq!(Solution::minimum_beautiful_substrings("0".to_string()), -1);
    }
}
