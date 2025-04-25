pub struct Solution {}

impl Solution {
    pub fn minimum_deletions(word: String, k: i32) -> i32 {
        let count = word.as_bytes().iter().fold([0; 26], |mut count, b| {
            count[(b - b'a') as usize] += 1;
            count
        });
        let mut result = std::i32::MAX;

        for i in 0..count.len() {
            result = result.min(
                count
                    .iter()
                    .map(|c| {
                        if c < &count[i] {
                            *c
                        } else {
                            (c - (count[i] + k)).max(0)
                        }
                    })
                    .sum::<i32>(),
            );
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3085() {
        assert_eq!(Solution::minimum_deletions("aabcaba".to_string(), 0), 3);
        assert_eq!(Solution::minimum_deletions("dabdcbdcdcd".to_string(), 2), 2);
        assert_eq!(Solution::minimum_deletions("aaabaaa".to_string(), 2), 1);
    }
}
