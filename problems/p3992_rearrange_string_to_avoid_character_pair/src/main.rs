pub struct Solution {}

impl Solution {
    pub fn rearrange_string(s: String, x: char, y: char) -> String {
        let mut chars = s.chars().collect::<Vec<_>>();
        chars.sort_unstable_by_key(|c| {
            if c == &y {
                -1
            } else {
                (*c as u8 - b'a') as i32
            }
        });
        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3992() {
        assert_eq!(
            Solution::rearrange_string("aabc".to_string(), 'a', 'c'),
            "cbaa".to_string()
        );
        assert_eq!(
            Solution::rearrange_string("dcab".to_string(), 'd', 'b'),
            "cabd".to_string()
        );
        assert_eq!(
            Solution::rearrange_string("axe".to_string(), 'o', 'x'),
            "axe".to_string()
        );
    }
}

fn main() {}
