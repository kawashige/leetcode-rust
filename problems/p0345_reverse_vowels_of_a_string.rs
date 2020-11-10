pub struct Solution {}

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        if s.is_empty() {
            return s;
        }

        let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mut chars = s.chars().collect::<Vec<char>>();
        let mut i = 0;
        let mut j = chars.len() as i32 - 1;
        while i < j {
            while i < chars.len() as i32 && !vowels.contains(&chars[i as usize]) {
                i += 1;
            }
            while -1 < j && !vowels.contains(&chars[j as usize]) {
                j -= 1;
            }

            if i < j && i < chars.len() as i32 && -1 < j {
                chars.swap(i as usize, j as usize);
            }
            i += 1;
            j -= 1;
        }
        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0345() {
        assert_eq!("Aa".to_string(), Solution::reverse_vowels("aA".to_string()));
        assert_eq!(
            "holle".to_string(),
            Solution::reverse_vowels("hello".to_string())
        );
        assert_eq!(
            "leotcede".to_string(),
            Solution::reverse_vowels("leetcode".to_string())
        );
        assert_eq!("".to_string(), Solution::reverse_vowels("".to_string()));
        assert_eq!(
            "qwd".to_string(),
            Solution::reverse_vowels("qwd".to_string())
        );
        assert_eq!("a".to_string(), Solution::reverse_vowels("a".to_string()));
    }
}
