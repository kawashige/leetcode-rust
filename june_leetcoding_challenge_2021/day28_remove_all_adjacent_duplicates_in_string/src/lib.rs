pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        s.chars()
            .fold(Vec::with_capacity(s.len()), |mut v, c| {
                if v.is_empty() || v.last() != Some(&c) {
                    v.push(c);
                } else {
                    v.pop();
                }
                v
            })
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day28() {
        assert_eq!(
            Solution::remove_duplicates("abbaca".to_string()),
            "ca".to_string()
        );
        assert_eq!(
            Solution::remove_duplicates("azxxzy".to_string()),
            "ay".to_string()
        );
    }
}
