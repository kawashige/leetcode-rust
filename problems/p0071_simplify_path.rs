pub struct Solution {}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut results = Vec::new();
        path.split('/')
            .filter(|s| s != &"." && s != &"")
            .for_each(|s| {
                if s == ".." {
                    results.pop();
                } else {
                    results.push(s);
                }
            });
        format!("/{}", results.join("/"))
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0071() {
        assert_eq!(
            "/home".to_string(),
            Solution::simplify_path("/home/".to_string())
        );
        assert_eq!("/".to_string(), Solution::simplify_path("/../".to_string()));
        assert_eq!(
            "/home/foo".to_string(),
            Solution::simplify_path("/home//foo/".to_string())
        );
        assert_eq!(
            "/c".to_string(),
            Solution::simplify_path("/a/./b/../../c/".to_string())
        );
        assert_eq!(
            "/c".to_string(),
            Solution::simplify_path("/a/../../b/../c//.//".to_string())
        );
        assert_eq!(
            "/a/b/c".to_string(),
            Solution::simplify_path("/a//b////c/d//././/..".to_string())
        );
    }
}
