pub struct Solution {}

impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        fn recurse(n1: u64, n2: u64, rest: &str) -> bool {
            if rest.is_empty() {
                return true;
            }
            let added = (n1 + n2).to_string();
            if rest.starts_with(&added) {
                recurse(n2, n1 + n2, &rest[added.len()..])
            } else {
                false
            }
        }

        for i in 0..num.len() {
            if num.starts_with("0") && 0 < i {
                break;
            }
            for j in (i + 1)..num.len() {
                if num[(i + 1)..].starts_with("0") && i + 1 < j {
                    break;
                }
                if j != num.len() - 1
                    && recurse(
                        num[..=i].parse::<u64>().unwrap(),
                        num[(i + 1)..=j].parse::<u64>().unwrap(),
                        &num[(j + 1)..],
                    )
                {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0306() {
        assert!(Solution::is_additive_number("000".to_string()));
        assert!(Solution::is_additive_number("101".to_string()));
        assert!(!Solution::is_additive_number("023".to_string()));
        assert!(!Solution::is_additive_number("1023".to_string()));
        assert!(Solution::is_additive_number("112358".to_string()));
        assert!(Solution::is_additive_number("199100199".to_string()));
        assert!(!Solution::is_additive_number("199".to_string()));
        assert!(Solution::is_additive_number("198019823962".to_string()));
        assert!(!Solution::is_additive_number("".to_string()));
    }
}
