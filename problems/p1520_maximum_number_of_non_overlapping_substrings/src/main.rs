use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        let mut chars = [[std::usize::MAX, std::usize::MIN]; 26];
        for i in 0..s.len() {
            let j = (s.as_bytes()[i] - b'a') as usize;
            chars[j][0] = chars[j][0].min(i);
            chars[j][1] = chars[j][1].max(i);
        }
        let mut set = HashSet::new();
        for i in 0..chars.len() {
            if chars[i][0] == std::usize::MAX {
                continue;
            }
            let (mut l, mut r) = (chars[i][0], chars[i][1]);
            let mut stack = (l..=r).collect::<Vec<_>>();
            while let Some(j) = stack.pop() {
                let k = (s.as_bytes()[j] - b'a') as usize;
                if chars[k][0] < l {
                    for x in chars[k][0]..l {
                        stack.push(x);
                    }
                    l = chars[k][0];
                }
                if r < chars[k][1] {
                    for x in r + 1..=chars[k][1] {
                        stack.push(x);
                    }
                    r = chars[k][1];
                }
            }
            set.insert((l, r));
        }
        let mut ranges = set.into_iter().collect::<Vec<_>>();
        ranges.sort_unstable_by_key(|r| (r.1 - r.0, r.0, r.1));

        let mut result: Vec<(usize, usize)> = Vec::new();
        for r in ranges {
            let mut is_overlap = false;
            for j in 0..result.len() {
                if !(r.1 < result[j].0 || result[j].1 < r.0) {
                    is_overlap = true;
                    break;
                }
            }
            if !is_overlap {
                result.push(r);
            }
        }

        result
            .into_iter()
            .map(|r| {
                (r.0..=r.1)
                    .map(|i| s.as_bytes()[i] as char)
                    .collect::<String>()
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1520() {
        assert_eq!(
            Solution::max_num_of_substrings("abab".to_string()),
            vec!["abab".to_string()]
        );
        assert_eq!(
            Solution::max_num_of_substrings("adefaddaccc".to_string()),
            vec!["e".to_string(), "f".to_string(), "ccc".to_string()]
        );
        assert_eq!(
            Solution::max_num_of_substrings("abbaccd".to_string()),
            vec!["d".to_string(), "bb".to_string(), "cc".to_string()]
        );
    }
}

fn main() {}
