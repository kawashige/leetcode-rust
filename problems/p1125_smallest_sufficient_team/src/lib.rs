use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn recurse(target: usize, people: &[usize], memo: &mut HashMap<usize, usize>) -> usize {
        if let Some(t) = memo.get(&target) {
            return *t;
        }
        if target == 0 {
            return 0;
        }
        let min_team = (0..people.len())
            .filter(|i| target & people[*i] != 0)
            .map(|i| 1 << i | Self::recurse((target ^ people[i]) & target, people, memo))
            .min_by_key(|t| t.count_ones())
            .unwrap();

        memo.insert(target, min_team);
        min_team
    }

    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let skill_indices = req_skills.into_iter().zip(0..).collect::<HashMap<_, _>>();
        let people = people
            .into_iter()
            .map(|skills| {
                skills
                    .into_iter()
                    .fold(0, |acc, skill| acc | 1 << skill_indices[&skill])
            })
            .collect::<Vec<_>>();
        let target = 2_usize.pow(skill_indices.len() as u32) - 1;

        let min_team = Self::recurse(target, &people, &mut HashMap::new());

        (0..people.len() as i32)
            .filter(|i| min_team & 1 << i != 0)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1125() {
        assert_eq!(
            Solution::smallest_sufficient_team(
                vec![
                    "mmcmnwacnhhdd".to_string(),
                    "vza".to_string(),
                    "mrxyc".to_string()
                ],
                vec![
                    vec!["mmcmnwacnhhdd".to_string()],
                    vec![],
                    vec![],
                    vec!["vza".to_string(), "mrxyc".to_string()]
                ]
            ),
            vec![0, 3]
        );
        assert_eq!(
            Solution::smallest_sufficient_team(
                vec![
                    "java".to_string(),
                    "nodejs".to_string(),
                    "reactjs".to_string()
                ],
                vec![
                    vec!["java".to_string()],
                    vec!["nodejs".to_string()],
                    vec!["nodejs".to_string(), "reactjs".to_string()]
                ]
            ),
            vec![0, 2]
        );
        assert_eq!(
            Solution::smallest_sufficient_team(
                vec![
                    "algorithms".to_string(),
                    "math".to_string(),
                    "java".to_string(),
                    "reactjs".to_string(),
                    "csharp".to_string(),
                    "aws".to_string()
                ],
                vec![
                    vec![
                        "algorithms".to_string(),
                        "math".to_string(),
                        "java".to_string()
                    ],
                    vec![
                        "algorithms".to_string(),
                        "math".to_string(),
                        "reactjs".to_string()
                    ],
                    vec!["java".to_string(), "csharp".to_string(), "aws".to_string()],
                    vec!["reactjs".to_string(), "csharp".to_string()],
                    vec!["csharp".to_string(), "math".to_string()],
                    vec!["aws".to_string(), "java".to_string()]
                ]
            ),
            vec![1, 2]
        );
    }
}
