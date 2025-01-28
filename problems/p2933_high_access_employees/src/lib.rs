use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_high_access_employees(access_times: Vec<Vec<String>>) -> Vec<String> {
        let mut access_times = access_times
            .into_iter()
            .map(|a| {
                (
                    a[0].clone(),
                    a[1][..2].parse::<usize>().unwrap() * 60 + a[1][2..].parse::<usize>().unwrap(),
                )
            })
            .collect::<Vec<_>>();
        access_times.sort_unstable();
        let map = access_times.into_iter().fold(HashMap::new(), |mut map, a| {
            (*map.entry(a.0).or_insert(Vec::new())).push(a.1);
            map
        });

        map.into_iter()
            .filter(|(_, v)| v.windows(3).any(|vv| vv[2] - vv[0] < 60))
            .map(|(k, _)| k)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2933() {
        assert_eq!(
            Solution::find_high_access_employees(vec![
                vec!["a".to_string(), "0549".to_string()],
                vec!["b".to_string(), "0457".to_string()],
                vec!["a".to_string(), "0532".to_string()],
                vec!["a".to_string(), "0621".to_string()],
                vec!["b".to_string(), "0540".to_string()]
            ]),
            vec!["a".to_string()]
        );
        assert_eq!(
            Solution::find_high_access_employees(vec![
                vec!["d".to_string(), "0002".to_string()],
                vec!["c".to_string(), "0808".to_string()],
                vec!["c".to_string(), "0829".to_string()],
                vec!["e".to_string(), "0215".to_string()],
                vec!["d".to_string(), "1508".to_string()],
                vec!["d".to_string(), "1444".to_string()],
                vec!["d".to_string(), "1410".to_string()],
                vec!["c".to_string(), "0809".to_string()]
            ]),
            vec!["c".to_string(), "d".to_string()]
        );
        assert_eq!(
            Solution::find_high_access_employees(vec![
                vec!["cd".to_string(), "1025".to_string()],
                vec!["ab".to_string(), "1025".to_string()],
                vec!["cd".to_string(), "1046".to_string()],
                vec!["cd".to_string(), "1055".to_string()],
                vec!["ab".to_string(), "1124".to_string()],
                vec!["ab".to_string(), "1120".to_string()]
            ]),
            vec!["ab".to_string(), "cd".to_string()]
        );
    }
}
