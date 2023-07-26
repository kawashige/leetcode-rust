pub struct Solution {}

impl Solution {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        let mut s = String::new();

        let w = encoded_text.len() / rows as usize;
        for i in 0..w {
            for j in 0..rows as usize {
                if w <= i + j {
                    break;
                }
                s.push(encoded_text.as_bytes()[w * j + i + j] as char);
            }
        }

        s.trim_end().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2075() {
        assert_eq!(
            Solution::decode_ciphertext("ch   ie   pr".to_string(), 3),
            "cipher".to_string()
        );
        assert_eq!(
            Solution::decode_ciphertext("iveo    eed   l te   olc".to_string(), 4),
            "i love leetcode".to_string()
        );
        assert_eq!(
            Solution::decode_ciphertext("coding".to_string(), 1),
            "coding".to_string()
        );
    }
}
