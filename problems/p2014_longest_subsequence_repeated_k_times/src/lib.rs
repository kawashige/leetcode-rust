pub struct Solution {}

impl Solution {
    pub fn recurse(
        subsequence: &mut Vec<char>,
        count: &mut Vec<usize>,
        l: usize,
        s: &str,
        k: usize,
    ) -> Option<String> {
        if subsequence.len() == l {
            let mut c = 0;
            let mut j = 0;
            for i in 0..s.len() {
                if s.as_bytes()[i] == subsequence[j] as u8 {
                    j += 1;
                    if j == subsequence.len() {
                        c += 1;
                        j = 0;
                    }
                }
            }
            if k <= c {
                let result = subsequence.iter().cloned().collect();
                return Some(result);
            } else {
                return None;
            }
        }

        for i in (0..count.len()).rev() {
            if count[i] == 0 {
                continue;
            }
            subsequence.push((i as u8 + b'a') as char);
            count[i] -= 1;
            if let Some(result) = Self::recurse(subsequence, count, l, s, k) {
                return Some(result);
            }
            subsequence.pop();
            count[i] += 1;
        }

        None
    }

    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        let max_len = s.len() / k as usize;
        let mut count = s.as_bytes().iter().fold(vec![0; 26], |mut count, b| {
            count[(b - b'a') as usize] += 1;
            count
        });
        count = count.into_iter().map(|c| c / k as usize).collect();

        for l in (1..=max_len).rev() {
            if let Some(result) = Self::recurse(&mut Vec::new(), &mut count, l, &s, k as usize) {
                return result;
            }
        }

        Default::default()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2014() {
        assert_eq!(
            Solution::longest_subsequence_repeated_k("letsleetcode".to_string(), 2),
            "let".to_string()
        );
        assert_eq!(
            Solution::longest_subsequence_repeated_k("bb".to_string(), 2),
            "b".to_string()
        );
        assert_eq!(
            Solution::longest_subsequence_repeated_k("ab".to_string(), 2),
            "".to_string()
        );
    }
}
