pub struct Solution {}

impl Solution {
    pub fn compress(s: &str) -> Vec<(char, usize)> {
        s.chars().fold(Vec::new(), |mut v, c| {
            if !v.is_empty() && v.last_mut().unwrap().0 == c {
                v.last_mut().unwrap().1 += 1;
            } else {
                v.push((c, 1))
            }
            v
        })
    }

    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        let compress_s = Self::compress(&s);
        words
            .into_iter()
            .filter(|w| {
                let compress_w = Self::compress(w);
                compress_s.len() == compress_w.len()
                    && (0..compress_w.len()).all(|i| {
                        compress_s[i].0 == compress_w[i].0
                            && compress_s[i].1 >= compress_w[i].1
                            && (compress_s[i].1 > 2 || compress_s[i].1 == compress_w[i].1)
                    })
            })
            .count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0809() {
        assert_eq!(
            Solution::expressive_words(
                "heeellooo".to_string(),
                vec!["hello".to_string(), "hi".to_string(), "helo".to_string()]
            ),
            1
        );
        assert_eq!(
            Solution::expressive_words(
                "".to_string(),
                vec!["".to_string(), "hi".to_string(), "helo".to_string()]
            ),
            1
        );
    }
}
