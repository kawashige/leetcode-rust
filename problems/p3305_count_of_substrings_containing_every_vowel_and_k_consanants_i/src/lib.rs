pub struct Solution {}

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i32 {
        let k = k as usize;
        let mut count = vec![[0; 26]; word.len() + 1];
        let mut result = 0;

        for i in 0..word.len() {
            for j in 0..26 {
                count[i + 1][j] = count[i][j];
            }
            count[i + 1][(word.as_bytes()[i] - b'a') as usize] += 1;

            for j in 0..i {
                let occurence = (0..26).fold([0; 2], |mut occurence, k| {
                    if [b'a', b'e', b'i', b'o', b'u'].contains(&(b'a' + k as u8)) {
                        occurence[0] += (0 < count[i + 1][k] - count[j][k]) as usize;
                    } else {
                        occurence[1] += count[i + 1][k] - count[j][k];
                    }
                    occurence
                });
                if occurence[0] == 5 && occurence[1] == k {
                    result += 1;
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
    fn test_3305() {
        assert_eq!(Solution::count_of_substrings("aeioqq".to_string(), 1), 0);
        assert_eq!(Solution::count_of_substrings("aeiou".to_string(), 0), 1);
        assert_eq!(
            Solution::count_of_substrings("ieaouqqieaouqq".to_string(), 1),
            3
        );
    }
}
