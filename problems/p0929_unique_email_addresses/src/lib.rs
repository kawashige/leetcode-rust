use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        emails
            .into_iter()
            .map(|email| {
                let mut sp = email.split("@");
                sp.next()
                    .unwrap()
                    .chars()
                    .filter(|c| c != &'.')
                    .take_while(|c| c != &'+')
                    .collect::<String>()
                    + "@"
                    + sp.next().unwrap()
            })
            .collect::<HashSet<_>>()
            .len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0929() {
        assert_eq!(
            Solution::num_unique_emails(vec![
                "test.email+alex@leetcode.com".to_string(),
                "test.email.leet+alex@code.com".to_string()
            ]),
            2
        );
        assert_eq!(
            Solution::num_unique_emails(vec![
                "test.email+alex@leetcode.com".to_string(),
                "test.e.mail+bob.cathy@leetcode.com".to_string(),
                "testemail+david@lee.tcode.com".to_string()
            ]),
            2
        );
        assert_eq!(
            Solution::num_unique_emails(vec![
                "a@leetcode.com".to_string(),
                "b@leetcode.com".to_string(),
                "c@leetcode.com".to_string()
            ]),
            3
        );
    }
}
