pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut counter = HashMap::new();
        for c in s.chars() {
            (*counter.entry(c).or_insert(0)) += 1;
        }
        let mut chars = s.chars().collect::<Vec<char>>();
        chars.sort_by(|a, b| counter[&b].cmp(&counter[&a]).then(a.cmp(&b)));
        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0451() {
        assert_eq!(
            "eert".to_string(),
            Solution::frequency_sort("tree".to_string())
        );
        assert_eq!(
            "aaaccc".to_string(),
            Solution::frequency_sort("cccaaa".to_string())
        );
        assert_eq!(
            "bbAa".to_string(),
            Solution::frequency_sort("Aabb".to_string())
        );
        assert_eq!("".to_string(), Solution::frequency_sort("".to_string()));
        assert_eq!("a".to_string(), Solution::frequency_sort("a".to_string()));
    }
}
