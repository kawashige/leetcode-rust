pub struct Solution {}

impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let mut result = Vec::new();

        for r in s.as_bytes()[0]..=s.as_bytes()[3] {
            for c in s.as_bytes()[1]..=s.as_bytes()[4] {
                result.push(format!("{}{}", r as char, c as char));
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2194() {
        assert_eq!(
            Solution::cells_in_range("K1:L2".to_string()),
            vec![
                "K1".to_string(),
                "K2".to_string(),
                "L1".to_string(),
                "L2".to_string()
            ]
        );
        assert_eq!(
            Solution::cells_in_range("A1:F1".to_string()),
            vec![
                "A1".to_string(),
                "B1".to_string(),
                "C1".to_string(),
                "D1".to_string(),
                "E1".to_string(),
                "F1".to_string()
            ]
        );
    }
}
