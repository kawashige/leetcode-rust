pub struct Solution {}

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        for l in (1..=str1.len().max(str2.len())).rev() {
            if str1.len() % l != 0
                || str2.len() % l != 0
                || (0..str1.len())
                    .step_by(l)
                    .any(|i| &str1[i..(i + l)] != &str1[..l])
                || (0..str2.len())
                    .step_by(l)
                    .any(|i| &str2[i..(i + l)] != &str1[..l])
            {
                continue;
            }

            return str1[..l].to_string();
        }

        String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1071() {
        assert_eq!(
            Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string()),
            "ABC".to_string()
        );
        assert_eq!(
            Solution::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()),
            "AB".to_string()
        );
        assert_eq!(
            Solution::gcd_of_strings("LEET".to_string(), "CODE".to_string()),
            "".to_string()
        );
        assert_eq!(
            Solution::gcd_of_strings("ABCDEF".to_string(), "ABC".to_string()),
            "".to_string()
        );
    }
}
