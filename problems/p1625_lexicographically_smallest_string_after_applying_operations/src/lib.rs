use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let mut generated = HashSet::new();
        let mut candidate = vec![s];

        while let Some(s) = candidate.pop() {
            if generated.contains(&s) {
                continue;
            }
            generated.insert(s.clone());

            let mut chars = s.chars().collect::<Vec<_>>();
            let added = chars
                .iter()
                .cloned()
                .enumerate()
                .map(|(i, c)| {
                    if i % 2 == 0 {
                        c
                    } else {
                        ((c as u8 - b'0' + a as u8) % 10 + b'0') as char
                    }
                })
                .collect::<String>();
            candidate.push(added);

            chars.rotate_right(b as usize);
            let rotated = chars.into_iter().collect::<String>();
            candidate.push(rotated);
        }

        generated.into_iter().min().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1625() {
        assert_eq!(
            Solution::find_lex_smallest_string("5525".to_string(), 9, 2),
            "2050".to_string()
        );
        assert_eq!(
            Solution::find_lex_smallest_string("74".to_string(), 5, 1),
            "24".to_string()
        );
        assert_eq!(
            Solution::find_lex_smallest_string("0011".to_string(), 4, 2),
            "0011".to_string()
        );
    }
}
