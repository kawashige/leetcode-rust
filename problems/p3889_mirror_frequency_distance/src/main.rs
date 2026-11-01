pub struct Solution {}

impl Solution {
    pub fn mirror_frequency(s: String) -> i32 {
        let mut char_freq: [i32; 26] = [0; 26];
        let mut digit_freq: [i32; 10] = [0; 10];

        for b in s.as_bytes() {
            if b.is_ascii_digit() {
                digit_freq[(b - b'0') as usize] += 1;
            } else {
                char_freq[(b - b'a') as usize] += 1;
            }
        }

        let mut result = 0;

        for i in 0..char_freq.len() {
            result += (char_freq[i] - char_freq[char_freq.len() - 1 - i]).abs();
        }
        for i in 0..digit_freq.len() {
            result += (digit_freq[i] - digit_freq[digit_freq.len() - 1 - i]).abs();
        }

        result / 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3889() {
        assert_eq!(Solution::mirror_frequency("ab1z9".to_string()), 3);
        assert_eq!(Solution::mirror_frequency("4m7n".to_string()), 2);
        assert_eq!(Solution::mirror_frequency("byby".to_string()), 0);
    }
}

fn main() {}
