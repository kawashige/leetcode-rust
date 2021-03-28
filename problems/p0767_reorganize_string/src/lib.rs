pub struct Solution {}

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut count = s
            .chars()
            .fold([0; 26], |mut count, c| {
                count[c as usize - 0x61] += 1;
                count
            })
            .iter()
            .cloned()
            .enumerate()
            .collect::<Vec<(usize, usize)>>();
        count.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        if s.len() - count[0].1 < count[0].1 - 1 {
            return String::new();
        }

        let mut result = vec![vec![(count[0].0 as u8 + 0x61) as char]; count[0].1];

        let mut j = 0;
        for i in 1..count.len() {
            for _ in 0..count[i].1 {
                result[j].push((count[i].0 as u8 + 0x61) as char);
                j = (j + 1) % result.len();
            }
        }

        result.into_iter().flatten().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0767() {
        assert_eq!(
            Solution::reorganize_string("a".to_string()),
            "a".to_string()
        );
        assert_eq!(
            Solution::reorganize_string("aab".to_string()),
            "aba".to_string()
        );
        assert_eq!(
            Solution::reorganize_string("aaab".to_string()),
            "".to_string()
        );
    }
}
