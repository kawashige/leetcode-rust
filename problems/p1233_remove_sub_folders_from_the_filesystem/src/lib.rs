pub struct Solution {}

impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        folder.sort_unstable_by(|a, b| b.cmp(a));

        let mut result = Vec::new();

        while !folder.is_empty() {
            result.push(folder.pop().unwrap());
            let prefix = format!("{}/", result.last().unwrap());
            while !folder.is_empty() && folder.last().unwrap().starts_with(&prefix) {
                folder.pop();
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1233() {
        assert_eq!(
            Solution::remove_subfolders(vec![
                "/a".to_string(),
                "/a/b".to_string(),
                "/c/d".to_string(),
                "/c/d/e".to_string(),
                "/c/f".to_string()
            ]),
            vec!["/a".to_string(), "/c/d".to_string(), "/c/f".to_string()]
        );
        assert_eq!(
            Solution::remove_subfolders(vec![
                "/a".to_string(),
                "/a/b/c".to_string(),
                "/a/b/d".to_string()
            ]),
            vec!["/a".to_string()]
        );
        assert_eq!(
            Solution::remove_subfolders(vec![
                "/a/b/c".to_string(),
                "/a/b/ca".to_string(),
                "/a/b/d".to_string()
            ]),
            vec![
                "/a/b/c".to_string(),
                "/a/b/ca".to_string(),
                "/a/b/d".to_string()
            ]
        );
    }
}
