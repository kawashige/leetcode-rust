pub struct Solution {}

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        let mut vowels_count = [0; 5];
        let mut total_vowels = 0;
        let mut consonants = 0;
        let mut prev_consonant = -1;
        let mut prev_consonants = vec![-1; word.len()];
        let vowels = [b'a', b'e', b'i', b'u', b'o'];
        let mut result = 0;
        let mut l = 0;

        for i in 0..word.len() {
            prev_consonants[i] = prev_consonant;
            if let Some(j) = vowels.iter().position(|v| v == &word.as_bytes()[i]) {
                vowels_count[j] += 1;
                if vowels_count[j] == 1 {
                    total_vowels += 1;
                }
            } else {
                prev_consonant = i as i64;
                consonants += 1;
            }

            if total_vowels == 5 && k <= consonants {
                while total_vowels == 5 && k <= consonants {
                    if let Some(j) = vowels.iter().position(|v| v == &word.as_bytes()[l]) {
                        if vowels_count[j] == 1 {
                            break;
                        }
                        vowels_count[j] -= 1;
                    } else {
                        if k == consonants {
                            break;
                        }
                        consonants -= 1;
                    }
                    l += 1;
                }
                if total_vowels == 5 && k == consonants {
                    result += l as i64 - prev_consonants[l];
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
    fn test_3306() {
        assert_eq!(Solution::count_of_substrings("aeioqq".to_string(), 1), 0);
        assert_eq!(Solution::count_of_substrings("aeiou".to_string(), 0), 1);
        assert_eq!(
            Solution::count_of_substrings("ieaouqqieaouqq".to_string(), 1),
            3
        );
    }
}
