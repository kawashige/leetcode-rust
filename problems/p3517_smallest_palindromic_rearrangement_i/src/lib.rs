pub struct Solution {}

impl Solution {
    pub fn smallest_palindrome(s: String) -> String {
        let count = s.as_bytes().iter().fold([0; 26], |mut count, b| {
            count[(b - b'a') as usize] += 1;
            count
        });

        let mut chars = Vec::with_capacity(s.len());
        let mut center = None;
        for i in 0..count.len() {
            if count[i] % 2 == 1 {
                center = Some((i as u8 + b'a') as char);
            }
            for _ in 0..count[i] / 2 {
                chars.push((i as u8 + b'a') as char);
            }
        }

        if let Some(c) = center {
            chars
                .iter()
                .chain(std::iter::once(&c))
                .chain(chars.iter().rev())
                .collect()
        } else {
            chars.iter().chain(chars.iter().rev()).collect()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3517() {
        assert_eq!(
            Solution::smallest_palindrome("z".to_string()),
            "z".to_string()
        );
        assert_eq!(
            Solution::smallest_palindrome("babab".to_string()),
            "abbba".to_string()
        );
        assert_eq!(
            Solution::smallest_palindrome("daccad".to_string()),
            "acddca".to_string()
        );
    }
}
