pub struct Solution {}

impl Solution {
    pub fn beautiful_substrings(s: String, k: i32) -> i32 {
        let k = k as usize;

        let mut count = vec![0; s.len() + 1];
        for i in 0..s.len() {
            count[i + 1] = count[i];
            if [b'a', b'e', b'i', b'o', b'u'].contains(&s.as_bytes()[i]) {
                count[i + 1] += 1;
            }
        }

        let mut result = 0;
        for i in 0..s.len() {
            for j in 0..=i {
                let vowels = count[i + 1] - count[j];
                let consonants = i - j + 1 - (count[i + 1] - count[j]);
                if vowels == consonants && (vowels * vowels) % k == 0 {
                    result += 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2947() {
        assert_eq!(Solution::beautiful_substrings("baeyh".to_string(), 2), 2);
        assert_eq!(Solution::beautiful_substrings("abba".to_string(), 1), 3);
        assert_eq!(Solution::beautiful_substrings("bcdf".to_string(), 1), 0);
    }
}
