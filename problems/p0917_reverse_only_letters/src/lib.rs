pub struct Solution {}

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut r = String::new();
        let bytes = s.as_bytes();
        let mut j = bytes.len();

        for i in 0..bytes.len() {
            if bytes[i].is_ascii_alphabetic() {
                j -= 1;
                while !bytes[j].is_ascii_alphabetic() {
                    j -= 1;
                }
                r.push(bytes[j] as char);
            } else {
                r.push(bytes[i] as char);
            }
        }

        r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0917() {
        assert_eq!(
            Solution::reverse_only_letters("ab-cd".to_string()),
            "dc-ba".to_string()
        );
        assert_eq!(
            Solution::reverse_only_letters("a-bC-dEf-ghIj".to_string()),
            "j-Ih-gfE-dCba".to_string()
        );
        assert_eq!(
            Solution::reverse_only_letters("Test1ng-Leet=code-Q!".to_string()),
            "Qedo1ct-eeLg=ntse-T!".to_string()
        );
    }
}
