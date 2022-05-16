pub struct Solution {}

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut i = 1;

        for t in target {
            while t != i {
                result.push("Push".to_string());
                result.push("Pop".to_string());
                i += 1;
            }
            result.push("Push".to_string());
            i += 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1441() {
        assert_eq!(
            Solution::build_array(vec![1, 3], 3),
            vec![
                "Push".to_string(),
                "Push".to_string(),
                "Pop".to_string(),
                "Push".to_string()
            ]
        );
        assert_eq!(
            Solution::build_array(vec![1, 2, 3], 3),
            vec!["Push".to_string(), "Push".to_string(), "Push".to_string()]
        );
        assert_eq!(
            Solution::build_array(vec![1, 2], 4),
            vec!["Push".to_string(), "Push".to_string()]
        );
    }
}
