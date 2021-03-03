pub struct Solution {}

impl Solution {
    pub fn to_lower_case(str: String) -> String {
        str.chars()
            .map(|c| {
                if (0x41..=0x5a).contains(&(c as u8)) {
                    (c as u8 + 0x20) as char
                } else {
                    c
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0709() {
        assert_eq!(
            Solution::to_lower_case("Hello".to_string()),
            "hello".to_string()
        );
        assert_eq!(
            Solution::to_lower_case("here".to_string()),
            "here".to_string()
        );
        assert_eq!(
            Solution::to_lower_case("LOVERY".to_string()),
            "lovery".to_string()
        );
    }
}
