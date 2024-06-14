use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn substring_xor_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut min_indices = HashMap::new();
        for i in 0..s.len() {
            if s.as_bytes()[i] == b'0' {
                if !min_indices.contains_key(&0) {
                    min_indices.insert(0, vec![i as i32, i as i32]);
                }
                continue;
            }
            for j in 0..=29.min(s.len() - 1 - i) {
                let val = i32::from_str_radix(&s[i..=i + j], 2).unwrap();
                if !min_indices.contains_key(&val) {
                    min_indices.insert(val, vec![i as i32, (i + j) as i32]);
                }
            }
        }

        queries
            .into_iter()
            .map(|q| {
                min_indices
                    .get(&(q[0] ^ q[1]))
                    .unwrap_or(&vec![-1, -1])
                    .clone()
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2564() {
        assert_eq!(
            Solution::substring_xor_queries("101101".to_string(), vec![vec![0, 5], vec![1, 2]]),
            vec![vec![0, 2], vec![2, 3]]
        );
        assert_eq!(
            Solution::substring_xor_queries("0101".to_string(), vec![vec![12, 8]]),
            vec![vec![-1, -1]]
        );
        assert_eq!(
            Solution::substring_xor_queries("1".to_string(), vec![vec![4, 5]]),
            vec![vec![0, 0]]
        );
    }
}
