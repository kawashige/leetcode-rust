pub struct Solution {}

impl Solution {
    pub fn minimum_string(a: String, b: String, c: String) -> String {
        let strings = vec![a, b, c];
        let mut results = vec![];

        for i in 0..strings.len() {
            for j in 0..strings.len() {
                if i == j {
                    continue;
                }
                for k in 0..strings.len() {
                    if i == k || j == k {
                        continue;
                    }
                    let mut str = strings[i].clone();
                    for l in [j, k].iter() {
                        if str.contains(&strings[*l]) {
                            continue;
                        }
                        if let Some(m) = (0..strings[*l].len())
                            .rev()
                            .find(|m| str.ends_with(&strings[*l][..=*m]))
                        {
                            str += &strings[*l][m + 1..];
                        } else {
                            str += &strings[*l];
                        }
                    }
                    results.push(str);
                }
            }
        }
        results.sort_unstable_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(&b)));
        results[0].clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2800() {
        assert_eq!(
            Solution::minimum_string("b".to_string(), "bbc".to_string(), "bcb".to_string()),
            "bbcb".to_string()
        );
        assert_eq!(
            Solution::minimum_string("b".to_string(), "a".to_string(), "ba".to_string()),
            "ba".to_string()
        );
        assert_eq!(
            Solution::minimum_string("abc".to_string(), "bca".to_string(), "aaa".to_string()),
            "aaabca".to_string()
        );
        assert_eq!(
            Solution::minimum_string("ab".to_string(), "ba".to_string(), "aba".to_string()),
            "aba".to_string()
        );
    }
}
