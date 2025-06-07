pub struct Solution {}

impl Solution {
    pub fn clear_stars(s: String) -> String {
        let mut deleted = vec![false; s.len()];
        let mut indices = vec![vec![]; 26];
        for i in 0..s.len() {
            if s.as_bytes()[i] == b'*' {
                let j = (0..indices.len())
                    .find(|j| !indices[*j].is_empty())
                    .unwrap();
                deleted[indices[j].pop().unwrap()] = true;
                deleted[i] = true;
            } else {
                indices[(s.as_bytes()[i] - b'a') as usize].push(i);
            }
        }

        (0..s.len())
            .filter_map(|i| {
                if deleted[i] {
                    None
                } else {
                    Some(s.as_bytes()[i] as char)
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3170() {
        assert_eq!(
            Solution::clear_stars("aaba*".to_string()),
            "aab".to_string()
        );
        assert_eq!(Solution::clear_stars("abc".to_string()), "abc".to_string());
    }
}
