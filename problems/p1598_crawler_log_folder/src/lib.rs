pub struct Solution {}

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        logs.into_iter()
            .fold(0_usize, |depth, log| match log.as_str() {
                "./" => depth,
                "../" => depth.saturating_sub(1),
                _ => depth + 1,
            }) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1598() {
        assert_eq!(
            Solution::min_operations(vec![
                "d1/".to_string(),
                "d2/".to_string(),
                "../".to_string(),
                "d21/".to_string(),
                "./".to_string()
            ]),
            2
        );
        assert_eq!(
            Solution::min_operations(vec![
                "d1/".to_string(),
                "d2/".to_string(),
                "./".to_string(),
                "d3/".to_string(),
                "../".to_string(),
                "d31/".to_string()
            ]),
            3
        );
        assert_eq!(
            Solution::min_operations(vec![
                "d1/".to_string(),
                "../".to_string(),
                "../".to_string(),
                "../".to_string()
            ]),
            0
        );
    }
}
