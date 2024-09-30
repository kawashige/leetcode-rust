pub struct Solution {}

impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let mut count = 0;
        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if words[i].as_bytes()[0] == words[j].as_bytes()[1]
                    && words[i].as_bytes()[1] == words[j].as_bytes()[0]
                {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2744() {
        assert_eq!(
            Solution::maximum_number_of_string_pairs(vec![
                "cd".to_string(),
                "ac".to_string(),
                "dc".to_string(),
                "ca".to_string(),
                "zz".to_string()
            ]),
            2
        );
        assert_eq!(
            Solution::maximum_number_of_string_pairs(vec![
                "ab".to_string(),
                "ba".to_string(),
                "cc".to_string()
            ]),
            1
        );
        assert_eq!(
            Solution::maximum_number_of_string_pairs(vec!["aa".to_string(), "ab".to_string()]),
            0
        );
    }
}
