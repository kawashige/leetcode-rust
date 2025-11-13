pub struct Solution {}

impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let mut r = 0;
        let mut count1 = [0; 26];
        let count2 = word2.as_bytes().iter().fold([0; 26], |mut count, b| {
            count[(b - b'a') as usize] += 1;
            count
        });
        let mut result = 0;
        count1[(word1.as_bytes()[0] - b'a') as usize] += 1;
        for l in 0..word1.len() {
            if 0 < l {
                count1[(word1.as_bytes()[l - 1] - b'a') as usize] -= 1;
            }
            if r < l {
                count1[(word1.as_bytes()[l] - b'a') as usize] += 1;
                r = l;
            }
            let mut is_valid = (0..26).all(|i| count2[i] <= count1[i]);
            while r + 1 < word1.len() && !is_valid {
                r += 1;
                count1[(word1.as_bytes()[r] - b'a') as usize] += 1;
                is_valid = (0..26).all(|i| count2[i] <= count1[i]);
            }
            if is_valid {
                result += word1.len() as i64 - r as i64;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3297() {
        assert_eq!(
            Solution::valid_substring_count("bcca".to_string(), "abc".to_string()),
            1
        );
        assert_eq!(
            Solution::valid_substring_count("abcabc".to_string(), "abc".to_string()),
            10
        );
        assert_eq!(
            Solution::valid_substring_count("abcabc".to_string(), "aaabc".to_string()),
            0
        );
    }
}
