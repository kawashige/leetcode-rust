pub struct Solution {}

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut lines = Vec::new();
        let mut words_len = Vec::new();
        let mut line = Vec::new();
        let mut len = 0;

        for word in words {
            if max_width < len + word.len() + line.len() {
                lines.push(line.clone());
                line.clear();
                words_len.push(len);
                len = 0;
            }
            len += word.len();
            line.push(word);
        }
        lines.push(line.clone());
        words_len.push(len);

        let mut result = Vec::new();

        for i in 0..lines.len() {
            let mut s = String::new();
            for j in 0..lines[i].len() {
                s += &lines[i][j];
                let spaces = if i == lines.len() - 1 {
                    if j == lines[i].len() - 1 {
                        max_width - s.len()
                    } else {
                        1
                    }
                } else if lines[i].len() == 1 {
                    max_width - s.len()
                } else if j < lines[i].len() - 1 {
                    if j < (max_width - words_len[i]) % (lines[i].len() - 1) {
                        (max_width - words_len[i] + lines[i].len() - 2) / (lines[i].len() - 1)
                    } else {
                        (max_width - words_len[i]) / (lines[i].len() - 1)
                    }
                } else {
                    0
                };
                for _ in 0..spaces {
                    s.push(' ')
                }
            }
            result.push(s)
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0068() {
        assert_eq!(
            Solution::full_justify(
                vec![
                    "This".to_string(),
                    "is".to_string(),
                    "an".to_string(),
                    "example".to_string(),
                    "of".to_string(),
                    "text".to_string(),
                    "justification.".to_string()
                ],
                16
            ),
            vec![
                "This    is    an".to_string(),
                "example  of text".to_string(),
                "justification.  ".to_string()
            ]
        );
        assert_eq!(
            Solution::full_justify(
                vec![
                    "What".to_string(),
                    "must".to_string(),
                    "be".to_string(),
                    "acknowledgment".to_string(),
                    "shall".to_string(),
                    "be".to_string()
                ],
                16
            ),
            vec![
                "What   must   be".to_string(),
                "acknowledgment  ".to_string(),
                "shall be        ".to_string()
            ]
        );
        assert_eq!(
            Solution::full_justify(
                vec![
                    "Science".to_string(),
                    "is".to_string(),
                    "what".to_string(),
                    "we".to_string(),
                    "understand".to_string(),
                    "well".to_string(),
                    "enough".to_string(),
                    "to".to_string(),
                    "explain".to_string(),
                    "to".to_string(),
                    "a".to_string(),
                    "computer.".to_string(),
                    "Art".to_string(),
                    "is".to_string(),
                    "everything".to_string(),
                    "else".to_string(),
                    "we".to_string(),
                    "do".to_string()
                ],
                20
            ),
            vec![
                "Science  is  what we".to_string(),
                "understand      well".to_string(),
                "enough to explain to".to_string(),
                "a  computer.  Art is".to_string(),
                "everything  else  we".to_string(),
                "do                  ".to_string()
            ]
        );
    }
}
