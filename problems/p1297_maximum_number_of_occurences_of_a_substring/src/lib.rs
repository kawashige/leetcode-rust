use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32 {
        let mut count = HashMap::new();

        for i in 0..s.len() {
            let mut substring = String::new();
            let mut chars: usize = 0;

            for j in 0..max_size as usize {
                if s.len() <= i + j {
                    break;
                }
                chars |= 1 << s.as_bytes()[i + j] - 0x61;
                if max_letters < chars.count_ones() as i32 {
                    break;
                }
                substring.push(s.as_bytes()[i + j] as char);

                if min_size as usize <= j + 1 {
                    *count.entry(&s[i..=(i + j)]).or_insert(0) += 1;
                }
            }
        }

        count.into_iter().map(|(_, v)| v).max().unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1297() {
        assert_eq!(Solution::max_freq("aababcaab".to_string(), 2, 3, 4), 2);
        assert_eq!(Solution::max_freq("aaaa".to_string(), 1, 3, 3), 2);
    }
}
