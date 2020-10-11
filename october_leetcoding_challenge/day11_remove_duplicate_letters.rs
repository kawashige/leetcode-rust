pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut map = s
            .chars()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (i, c)| {
                acc.entry(c).or_insert(Vec::new()).push(i);
                acc
            })
            .into_iter()
            .collect::<Vec<(char, Vec<usize>)>>();

        map.sort_by(|a, b| a.1[0].cmp(&b.1[0]));

        let mut changed = true;
        while changed {
            changed = false;
            for i in 0..(map.len() - 1) {
                if map[i].1.len() > 1 {
                    if map[i + 1].0 < map[i].0 {
                        if let Some(n) = map[i].1.iter().find(|n| map[i + 1].1[0] < **n) {
                            map[i].1[0] = *n;
                            changed = true;
                            break;
                        }
                    }
                }
            }
            map.sort_by(|a, b| a.1[0].cmp(&b.1[0]));
        }
        println!("{:?}", map);

        map.into_iter().map(|(c, _)| c).collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day11() {
        assert_eq!(
            "abc".to_string(),
            Solution::remove_duplicate_letters("bcabc".to_string())
        );
        assert_eq!(
            "acdb".to_string(),
            Solution::remove_duplicate_letters("cbacdcbc".to_string())
        );
        assert_eq!(
            "zacdb".to_string(),
            Solution::remove_duplicate_letters("zcbacdcbca".to_string())
        );
    }
}
