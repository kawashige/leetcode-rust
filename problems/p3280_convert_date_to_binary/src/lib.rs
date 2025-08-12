pub struct Solution {}

impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        date.split('-')
            .map(|d| format!("{:b}", d.parse::<usize>().unwrap()))
            .collect::<Vec<_>>()
            .join("-")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3280() {
        assert_eq!(
            Solution::convert_date_to_binary("2080-02-29".to_string()),
            "100000100000-10-11101".to_string()
        );
        assert_eq!(
            Solution::convert_date_to_binary("1900-01-01".to_string()),
            "11101101100-1-1".to_string()
        );
    }
}
