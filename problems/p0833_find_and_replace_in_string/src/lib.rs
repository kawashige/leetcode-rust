pub struct Solution {}

impl Solution {
    pub fn find_replace_string(
        s: String,
        indexes: Vec<i32>,
        sources: Vec<String>,
        targets: Vec<String>,
    ) -> String {
        let mut replaces = (0..indexes.len())
            .map(|i| (indexes[i] as usize, &sources[i], &targets[i]))
            .collect::<Vec<(usize, &String, &String)>>();
        replaces.sort_unstable();

        let mut result = String::new();
        let mut i = 0;

        for (j, source, target) in replaces {
            result += &s[i..j];
            result += if &s[j..(j + source.len())] == source {
                target
            } else {
                &s[j..(j + source.len())]
            };
            i = j + source.len();
        }

        result + &s[i..]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0833() {
        assert_eq!(
            Solution::find_replace_string(
                "abcd".to_string(),
                vec![0, 2],
                vec!["a".to_string(), "cd".to_string()],
                vec!["eee".to_string(), "ffff".to_string()]
            ),
            "eeebffff".to_string()
        );

        assert_eq!(
            Solution::find_replace_string(
                "abcd".to_string(),
                vec![0, 2],
                vec!["ab".to_string(), "ec".to_string()],
                vec!["eee".to_string(), "ffff".to_string()]
            ),
            "eeecd".to_string()
        );
    }
}
