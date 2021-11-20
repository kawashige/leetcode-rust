use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
        let mut set = HashSet::new();
        for i in 0..s.len() {
            if s[i..].starts_with("0") {
                continue;
            }
            for j in i..(i + 31).min(s.len()) {
                set.insert(i32::from_str_radix(&s[i..=j], 2).unwrap());
            }
        }

        let mut v = set.into_iter().collect::<Vec<_>>();
        v.push(0);
        v.sort_unstable();

        (n as usize) < v.len() && v[n as usize] == n
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1016() {
        assert!(!Solution::query_string(
            "0110100100110010110101011001101100111101".to_string(),
            20
        ));
        assert!(Solution::query_string("0110".to_string(), 3));
        assert!(!Solution::query_string("0110".to_string(), 4));
    }
}
