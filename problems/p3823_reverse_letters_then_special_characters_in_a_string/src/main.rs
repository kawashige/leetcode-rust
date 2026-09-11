pub struct Solution {}

impl Solution {
    pub fn reverse_by_type(s: String) -> String {
        let mut chars = vec![];
        let mut specials = vec![];

        for i in 0..s.len() {
            if s.as_bytes()[i].is_ascii_alphabetic() {
                chars.push(s.as_bytes()[i]);
            } else {
                specials.push(s.as_bytes()[i]);
            }
        }

        let mut result = String::new();

        for i in 0..s.len() {
            result.push(if s.as_bytes()[i].is_ascii_alphabetic() {
                chars.pop().unwrap() as char
            } else {
                specials.pop().unwrap() as char
            });
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3823() {
        assert_eq!(
            Solution::reverse_by_type(")ebc#da@f(".to_string()),
            "(fad@cb#e)".to_string()
        );
        assert_eq!(Solution::reverse_by_type("z".to_string()), "z".to_string());
        assert_eq!(
            Solution::reverse_by_type("!@#$%^&*()".to_string()),
            ")(*&^%$#@!".to_string()
        );
    }
}

fn main() {}
