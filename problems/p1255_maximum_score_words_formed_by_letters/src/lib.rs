pub struct Solution {}

impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let letters = letters.into_iter().fold([0; 26], |mut count, c| {
            count[(c as u8 - b'a') as usize] += 1;
            count
        });
        let words = words
            .into_iter()
            .map(|w| {
                w.as_bytes().iter().fold([0; 26], |mut count, b| {
                    count[(b - b'a') as usize] += 1;
                    count
                })
            })
            .collect::<Vec<_>>();

        let mut max_score = 0;
        for i in 0..2_usize.pow(words.len() as u32) {
            let mut scores = 0;
            let mut letter_count = vec![0; 26];
            for j in 0..=words.len() {
                if i & 1 << j == 0 {
                    continue;
                }
                for k in 0..26 {
                    letter_count[k] += words[j][k];
                    scores += score[k] * words[j][k];
                    if letters[k] < letter_count[k] {
                        scores = 0;
                        break;
                    }
                }
            }
            max_score = max_score.max(scores);
        }

        max_score
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1255() {
        assert_eq!(
            Solution::max_score_words(
                vec![
                    "dog".to_string(),
                    "cat".to_string(),
                    "dad".to_string(),
                    "good".to_string()
                ],
                vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'],
                vec![1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ),
            23
        );
        assert_eq!(
            Solution::max_score_words(
                vec![
                    "xxxz".to_string(),
                    "ax".to_string(),
                    "bx".to_string(),
                    "cx".to_string()
                ],
                vec!['z', 'a', 'b', 'c', 'x', 'x', 'x'],
                vec![4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10]
            ),
            27
        );
        assert_eq!(
            Solution::max_score_words(
                vec!["leetcode".to_string()],
                vec!['l', 'e', 't', 'c', 'o', 'd'],
                vec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]
            ),
            0
        );
    }
}
