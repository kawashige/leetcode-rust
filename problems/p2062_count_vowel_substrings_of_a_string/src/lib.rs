pub struct Solution {}

impl Solution {
    pub fn count_vowel_substrings(word: String) -> i32 {
        let vowels = [b'a', b'e', b'i', b'o', b'u'];
        let mut counts = vec![[0; 5]; word.len() + 1];
        let mut count = [0; 5];
        for i in 0..word.len() {
            if let Some(j) = (0..vowels.len()).find(|j| vowels[*j] == word.as_bytes()[i]) {
                count[j] += 1;
            }
            counts[i + 1] = count.clone();
        }

        let mut count = 0;

        for i in 0..word.len() {
            for j in i..word.len() {
                if (0..vowels.len()).all(|k| 0 < counts[j + 1][k] - counts[i][k])
                    && (0..vowels.len())
                        .map(|k| counts[j + 1][k] - counts[i][k])
                        .sum::<usize>()
                        == j + 1 - i
                {
                    count += 1;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2062() {
        assert_eq!(Solution::count_vowel_substrings("aeiouu".to_string()), 2);
        assert_eq!(
            Solution::count_vowel_substrings("unicornarihan".to_string()),
            0
        );
        assert_eq!(
            Solution::count_vowel_substrings("cuaieuouac".to_string()),
            7
        );
    }
}
