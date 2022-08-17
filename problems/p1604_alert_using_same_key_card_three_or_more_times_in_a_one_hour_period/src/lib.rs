pub struct Solution {}

impl Solution {
    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        let mut keys = key_name
            .into_iter()
            .zip(key_time.into_iter().map(|time| {
                let mut sp = time.split(':');
                sp.next().unwrap().parse::<usize>().unwrap() * 60
                    + sp.next().unwrap().parse::<usize>().unwrap()
            }))
            .collect::<Vec<_>>();
        keys.sort_unstable();

        let mut result = Vec::new();

        for i in 2..keys.len() {
            if result.last() == Some(&keys[i].0) {
                continue;
            }
            if keys[i - 2].0 == keys[i].0 && keys[i].1 <= keys[i - 2].1 + 60 {
                result.push(keys[i].0.clone());
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1604() {
        assert_eq!(
            Solution::alert_names(
                vec![
                    "daniel".to_string(),
                    "daniel".to_string(),
                    "daniel".to_string(),
                    "luis".to_string(),
                    "luis".to_string(),
                    "luis".to_string(),
                    "luis".to_string()
                ],
                vec![
                    "10:00".to_string(),
                    "10:40".to_string(),
                    "11:00".to_string(),
                    "09:00".to_string(),
                    "11:00".to_string(),
                    "13:00".to_string(),
                    "15:00".to_string()
                ]
            ),
            vec!["daniel".to_string()]
        );
        assert_eq!(
            Solution::alert_names(
                vec![
                    "alice".to_string(),
                    "alice".to_string(),
                    "alice".to_string(),
                    "bob".to_string(),
                    "bob".to_string(),
                    "bob".to_string(),
                    "bob".to_string()
                ],
                vec![
                    "12:01".to_string(),
                    "12:00".to_string(),
                    "18:00".to_string(),
                    "21:00".to_string(),
                    "21:20".to_string(),
                    "21:30".to_string(),
                    "23:00".to_string()
                ]
            ),
            vec!["bob".to_string()]
        );
    }
}
