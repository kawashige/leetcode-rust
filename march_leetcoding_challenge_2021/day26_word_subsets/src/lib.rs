pub struct Solution {}

impl Solution {
    pub fn count(s: &str) -> [usize; 26] {
        s.chars().fold([0; 26], |mut count, c| {
            count[c as usize - 0x61] += 1;
            count
        })
    }

    pub fn word_subsets(a: Vec<String>, b: Vec<String>) -> Vec<String> {
        let b_count = b.into_iter().fold([0; 26], |mut b_count, w| {
            let count = Self::count(&w);
            for i in 0..26 {
                b_count[i] = std::cmp::max(b_count[i], count[i]);
            }
            b_count
        });

        a.into_iter()
            .filter(|w| {
                let a_count = Self::count(w);
                (0..26).all(|i| a_count[i] >= b_count[i])
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day26() {
        assert_eq!(
            Solution::word_subsets(
                vec![
                    "amazon".to_string(),
                    "apple".to_string(),
                    "facebook".to_string(),
                    "google".to_string(),
                    "leetcode".to_string()
                ],
                vec!["l".to_string(), "e".to_string()]
            ),
            vec![
                "apple".to_string(),
                "google".to_string(),
                "leetcode".to_string()
            ]
        );

        assert_eq!(
            Solution::word_subsets(
                vec![
                    "amazon".to_string(),
                    "apple".to_string(),
                    "facebook".to_string(),
                    "google".to_string(),
                    "leetcode".to_string()
                ],
                vec!["e".to_string(), "oo".to_string()]
            ),
            vec!["facebook".to_string(), "google".to_string(),]
        );

        assert_eq!(
            Solution::word_subsets(
                vec![
                    "amazon".to_string(),
                    "apple".to_string(),
                    "facebook".to_string(),
                    "google".to_string(),
                    "leetcode".to_string()
                ],
                vec!["lo".to_string(), "eo".to_string()]
            ),
            vec!["google".to_string(), "leetcode".to_string(),]
        );

        assert_eq!(
            Solution::word_subsets(
                vec![
                    "amazon".to_string(),
                    "apple".to_string(),
                    "facebook".to_string(),
                    "google".to_string(),
                    "leetcode".to_string()
                ],
                vec!["ec".to_string(), "oc".to_string(), "ceo".to_string()]
            ),
            vec!["facebook".to_string(), "leetcode".to_string(),]
        );
    }
}
