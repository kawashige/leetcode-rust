use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        paths
            .into_iter()
            .fold(HashMap::new(), |mut map, path| {
                let mut splitted = path.split(' ');
                let dir = splitted.next().unwrap();
                for s in splitted {
                    let file = s.split('(').collect::<Vec<&str>>();
                    (*map
                        .entry(file[1].trim_end_matches(")").to_string())
                        .or_insert(Vec::new()))
                    .push(format!("{}/{}", dir, file[0]));
                }
                map
            })
            .values()
            .filter(|v| v.len() > 1)
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0609() {
        assert_eq!(
            Solution::find_duplicate(vec![
                "root/a 1.txt(abcd) 2.txt(efgh)".to_string(),
                "root/c 3.txt(abcd)".to_string(),
                "root/c/d 4.txt(efgh)".to_string(),
                "root 4.txt(efgh)".to_string()
            ]),
            vec![
                vec![
                    "root/a/2.txt".to_string(),
                    "root/c/d/4.txt".to_string(),
                    "root/4.txt".to_string()
                ],
                vec!["root/a/1.txt".to_string(), "root/c/3.txt".to_string()]
            ]
        )
    }
}
