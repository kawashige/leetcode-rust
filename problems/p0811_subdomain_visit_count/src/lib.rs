use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        cpdomains
            .into_iter()
            .fold(HashMap::new(), |mut map, domain| {
                let mut sp = domain.split_ascii_whitespace();
                let count = sp.next().unwrap().parse::<usize>().unwrap();
                let subdomains = sp.next().unwrap().split(".").collect::<Vec<&str>>();
                for i in 0..subdomains.len() {
                    *map.entry(subdomains[i..].join(".")).or_insert(0) += count;
                }
                map
            })
            .into_iter()
            .map(|(k, c)| format!("{} {}", c, k))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0811() {
        assert_eq!(
            Solution::subdomain_visits(vec!["9001 discuss.leetcode.com".to_string()]),
            vec![
                "9001 discuss.leetcode.com".to_string(),
                "9001 leetcode.com".to_string(),
                "9001 com".to_string()
            ]
        );
        assert_eq!(
            Solution::subdomain_visits(vec![
                "900 google.mail.com".to_string(),
                "50 yahoo.com".to_string(),
                "1 intel.mail.com".to_string(),
                "5 wiki.org".to_string()
            ]),
            vec![
                "901 mail.com".to_string(),
                "50 yahoo.com".to_string(),
                "900 google.mail.com".to_string(),
                "5 wiki.org".to_string(),
                "5 org".to_string(),
                "1 intel.mail.com".to_string(),
                "951 com".to_string()
            ]
        );
    }
}
