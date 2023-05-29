pub struct Solution {}

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        operations.into_iter().fold(0, |acc, op| match op.as_str() {
            "++X" | "X++" => acc + 1,
            _ => acc - 1,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2011() {
        assert_eq!(
            Solution::final_value_after_operations(vec![
                "--X".to_string(),
                "X++".to_string(),
                "X++".to_string()
            ]),
            1
        );
        assert_eq!(
            Solution::final_value_after_operations(vec![
                "++X".to_string(),
                "++X".to_string(),
                "X++".to_string()
            ]),
            3
        );
        assert_eq!(
            Solution::final_value_after_operations(vec![
                "X++".to_string(),
                "++X".to_string(),
                "--X".to_string(),
                "X--".to_string()
            ]),
            0
        );
    }
}
