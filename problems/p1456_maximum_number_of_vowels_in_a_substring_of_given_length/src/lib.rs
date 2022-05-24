pub struct Solution {}

impl Solution {
    pub fn vowel_count(b: &u8) -> i32 {
        if [b'a', b'e', b'i', b'o', b'u'].contains(b) {
            1
        } else {
            0
        }
    }

    pub fn max_vowels(s: String, k: i32) -> i32 {
        let mut count: i32 = s.as_bytes()[..k as usize]
            .iter()
            .map(|b| Self::vowel_count(b))
            .sum();

        let mut max = count;
        for i in k as usize..s.len() {
            count += Self::vowel_count(&s.as_bytes()[i])
                - Self::vowel_count(&s.as_bytes()[i - k as usize]);
            max = max.max(count);
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1456() {
        assert_eq!(Solution::max_vowels("abciiidef".to_string(), 3), 3);
        assert_eq!(Solution::max_vowels("aeiou".to_string(), 2), 2);
        assert_eq!(Solution::max_vowels("leetcode".to_string(), 3), 2);
    }
}
