pub struct Solution {}

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let mut table = [26; 26];
        let mut i = 0;
        for b in key.as_bytes().iter() {
            if b != &b' ' && table[(b - b'a') as usize] == 26 {
                table[(b - b'a') as usize] = i;
                i += 1;
                if i == 26 {
                    break;
                }
            }
        }

        message
            .as_bytes()
            .iter()
            .map(|b| {
                (if b == &b' ' {
                    *b
                } else {
                    table[(b - b'a') as usize] + b'a'
                }) as char
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2325() {
        assert_eq!(
            Solution::decode_message(
                "the quick brown fox jumps over the lazy dog".to_string(),
                "vkbs bs t suepuv".to_string()
            ),
            "this is a secret".to_string()
        );
        assert_eq!(
            Solution::decode_message(
                "eljuxhpwnyrdgtqkviszcfmabo".to_string(),
                "zwx hnfx lqantp mnoeius ycgk vcnjrdb".to_string()
            ),
            "the five boxing wizards jump quickly".to_string()
        );
    }
}
