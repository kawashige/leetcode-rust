pub struct Solution {}

impl Solution {
    pub fn dfs(
        words: &Vec<String>,
        overlapped: &Vec<Vec<usize>>,
        prev_i: Option<usize>,
        used: usize,
        min_s: &mut String,
        s: String,
    ) {
        if used == (1 << words.len()) - 1 {
            *min_s = s.clone();
            return;
        }

        for i in 0..words.len() {
            if used & 1 << i > 0 {
                continue;
            }

            let s = format!(
                "{}{}",
                s,
                &words[i][prev_i.map_or_else(|| 0, |j| overlapped[j][i])..]
            );

            if s.len() > min_s.len() {
                continue;
            }

            Self::dfs(words, overlapped, i.into(), used | 1 << i, min_s, s);
        }
    }

    pub fn shortest_superstring(words: Vec<String>) -> String {
        let mut overlapped = vec![vec![0; words.len()]; words.len()];
        for i in 0..words.len() {
            for j in 0..words.len() {
                if i == j {
                    continue;
                }
                let bytes_i = words[i].as_bytes();
                let bytes_j = words[j].as_bytes();
                overlapped[i][j] = (1..std::cmp::min(words[i].len(), words[j].len()))
                    .rev()
                    .find(|k| bytes_i[(bytes_i.len() - k)..] == bytes_j[..*k])
                    .unwrap_or(0);
            }
        }

        // println!("{:?}", overlapped);

        let mut s = std::iter::repeat(".").take(20 * 12 + 1).collect();
        Self::dfs(&words, &overlapped, None, 0, &mut s, String::new());
        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day23() {
        assert_eq!(
            Solution::shortest_superstring(vec![
                "sssv".to_string(),
                "svq".to_string(),
                "dskss".to_string(),
                "sksss".to_string()
            ]),
            "dsksssvq".to_string()
        );
        assert_eq!(
            Solution::shortest_superstring(vec![
                "alex".to_string(),
                "loves".to_string(),
                "leetcode".to_string()
            ]),
            "leetcodelovesalex".to_string()
        );

        assert_eq!(
            Solution::shortest_superstring(vec![
                "catg".to_string(),
                "ctaagt".to_string(),
                "gcta".to_string(),
                "ttca".to_string(),
                "atgcatc".to_string()
            ]),
            "gctaagttcatgcatc".to_string()
        );
    }
}
