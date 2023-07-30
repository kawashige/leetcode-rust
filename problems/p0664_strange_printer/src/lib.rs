use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn recurse(
        l: usize,
        r: usize,
        b: u8,
        bytes: &[u8],
        memo: &mut HashMap<(usize, usize, u8), i32>,
    ) -> i32 {
        if let Some(step) = memo.get(&(l, r, b)) {
            return *step;
        }
        let mut min = std::i32::MAX;

        for i in l..=r {
            let base = if i == l {
                0
            } else {
                Self::recurse(l, i - 1, b, bytes, memo)
            };
            for j in (i..=r).filter(|j| bytes[i] == bytes[*j]) {
                let step =
                    base + if j == i {
                        0
                    } else {
                        Self::recurse(i + 1, j - 1, bytes[i], bytes, memo)
                    } + if j == r {
                        0
                    } else {
                        Self::recurse(j + 1, r, bytes[i], bytes, memo)
                    } + if bytes[i] == b { 0 } else { 1 };
                min = min.min(step);
            }
        }

        memo.insert((l, r, b), min);

        min
    }

    pub fn strange_printer(s: String) -> i32 {
        let mut chars = Vec::new();
        for b in s.as_bytes() {
            if chars.last() != Some(b) {
                chars.push(*b);
            }
        }

        Self::recurse(0, chars.len() - 1, 'A' as u8, &chars, &mut HashMap::new())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0664() {
        assert_eq!(
            Solution::strange_printer("dddccbdbababaddcbcaabdbdddcccddbbaabddb".to_string()),
            15
        );
        assert_eq!(Solution::strange_printer("abbbaa".to_string()), 2);
        assert_eq!(Solution::strange_printer("tbgtgb".to_string()), 4);
        assert_eq!(Solution::strange_printer("abcabc".to_string()), 5);
        assert_eq!(Solution::strange_printer("aaabbb".to_string()), 2);
        assert_eq!(Solution::strange_printer("aba".to_string()), 2);
    }
}
