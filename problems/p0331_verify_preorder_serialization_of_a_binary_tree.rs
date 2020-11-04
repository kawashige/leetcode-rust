pub struct Solution {}

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        fn recurse(values: &[&str], start: usize) -> (bool, usize) {
            if values.len() <= start {
                return (false, start);
            } else if values[start] == "#" {
                return (true, start);
            } else if values[start].parse::<i32>().is_err() {
                return (false, start);
            }

            let (left_result, left_end) = recurse(values, start + 1);
            if !left_result {
                return (false, left_end);
            }
            recurse(values, left_end + 1)
        }

        if preorder.is_empty() {
            return false;
        }
        let values = preorder.split(",").collect::<Vec<&str>>();
        let (result, end) = recurse(&values, 0);
        result && end == values.len() - 1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0331() {
        assert!(Solution::is_valid_serialization(
            "9,3,4,#,#,1,#,#,2,#,6,#,#".to_string()
        ));
        assert!(!Solution::is_valid_serialization("1,#".to_string()));
        assert!(!Solution::is_valid_serialization("9,#,#,1".to_string()));
    }
}
