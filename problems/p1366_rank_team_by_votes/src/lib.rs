pub struct Solution {}

impl Solution {
    pub fn rank_teams(votes: Vec<String>) -> String {
        let count = votes.into_iter().fold([[0; 26]; 26], |mut count, vote| {
            for (r, b) in vote.as_bytes().iter().enumerate() {
                count[*b as usize - 0x41][r] += 1;
            }
            count
        });
        let mut count = count
            .into_iter()
            .enumerate()
            .map(|(i, count)| (count, i))
            .collect::<Vec<_>>();

        count.sort_unstable_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        count
            .into_iter()
            .filter_map(|(count, i)| {
                if count.iter().all(|c| c == &0) {
                    None
                } else {
                    Some((i as u8 + 0x41) as char)
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1366() {
        assert_eq!(
            Solution::rank_teams(vec![
                "ABC".to_string(),
                "ACB".to_string(),
                "ABC".to_string(),
                "ACB".to_string(),
                "ACB".to_string()
            ]),
            "ACB".to_string()
        );
        assert_eq!(
            Solution::rank_teams(vec!["WXYZ".to_string(), "XYZW".to_string()]),
            "XWYZ".to_string()
        );
        assert_eq!(
            Solution::rank_teams(vec!["ZMNAGUEDSJYLBOPHRQICWFXTVK".to_string()]),
            "ZMNAGUEDSJYLBOPHRQICWFXTVK".to_string()
        );
        assert_eq!(
            Solution::rank_teams(vec![
                "BCA".to_string(),
                "CAB".to_string(),
                "CBA".to_string(),
                "ABC".to_string(),
                "ACB".to_string(),
                "BAC".to_string()
            ]),
            "ABC".to_string()
        )
    }
}
