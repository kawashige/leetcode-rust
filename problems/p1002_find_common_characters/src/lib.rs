pub struct Solution {}

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let count = words.iter().fold([100; 26], |mut count, word| {
            let mut tmp = [0; 26];
            for c in word.as_bytes() {
                tmp[*c as usize - 0x61] += 1;
            }
            for i in 0..26 {
                count[i] = count[i].min(tmp[i]);
            }
            count
        });

        (0..26).fold(Vec::new(), |mut r, i| {
            for _ in 0..count[i] {
                r.push(((i as u8 + 0x61) as char).to_string());
            }
            r
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1002() {
        assert_eq!(
            Solution::common_chars(vec![
                "bella".to_string(),
                "label".to_string(),
                "roller".to_string()
            ]),
            vec!["e".to_string(), "l".to_string(), "l".to_string()]
        );
        assert_eq!(
            Solution::common_chars(vec![
                "cool".to_string(),
                "lock".to_string(),
                "cook".to_string()
            ]),
            vec!["c".to_string(), "o".to_string()]
        );
    }
}
