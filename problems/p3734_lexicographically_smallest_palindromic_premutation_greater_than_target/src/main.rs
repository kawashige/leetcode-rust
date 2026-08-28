pub struct Solution {}

impl Solution {
    pub fn back_tracking(
        count: &mut [i32; 26],
        first_half: &mut Vec<char>,
        target: &str,
    ) -> String {
        if first_half.len() == target.len() / 2 {
            if let Some(c) = (0..count.len()).find(|i| count[*i] == 1) {
                let s = first_half
                    .iter()
                    .cloned()
                    .chain(std::iter::once((c as u8 + b'a') as char))
                    .chain(first_half.iter().rev().cloned())
                    .collect::<String>();

                if s.as_str() <= target {
                    return String::new();
                } else {
                    return s;
                }
            } else {
                let s = first_half
                    .iter()
                    .cloned()
                    .chain(first_half.iter().rev().cloned())
                    .collect::<String>();

                if s.as_str() <= target {
                    return String::new();
                } else {
                    return s;
                }
            }
        }
        if first_half.iter().cloned().collect::<String>().as_str() < &target[..first_half.len()] {
            return String::new();
        }

        for i in 0..count.len() {
            if 1 < count[i] {
                count[i] -= 2;
                first_half.push((i as u8 + b'a') as char);
                let result = Self::back_tracking(count, first_half, target);
                if !result.is_empty() {
                    return result;
                }
                count[i] += 2;
                first_half.pop();
            }
        }

        String::new()
    }

    pub fn lex_palindromic_permutation(s: String, target: String) -> String {
        let mut count = s.as_bytes().iter().fold([0; 26], |mut count, b| {
            count[(b - b'a') as usize] += 1;
            count
        });

        let mut odd_indices = Vec::new();
        for i in 0..count.len() {
            if count[i] % 2 == 1 {
                odd_indices.push(i);
            }
        }
        if 1 < odd_indices.len() {
            return String::new();
        }

        Self::back_tracking(&mut count, &mut Vec::new(), &target)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3734() {
        assert_eq!(
            Solution::lex_palindromic_permutation("baba".to_string(), "abba".to_string()),
            "baab".to_string()
        );
        assert_eq!(
            Solution::lex_palindromic_permutation("baba".to_string(), "bbaa".to_string()),
            "".to_string()
        );
        assert_eq!(
            Solution::lex_palindromic_permutation("abc".to_string(), "abb".to_string()),
            "".to_string()
        );
        assert_eq!(
            Solution::lex_palindromic_permutation("aac".to_string(), "abb".to_string()),
            "aca".to_string()
        );
    }
}

fn main() {}
