pub struct Solution {}

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut pairs = vec![vec![]; 26];
        for i in 0..s1.len() {
            let i1 = (s1.as_bytes()[i] - b'a') as usize;
            let i2 = (s2.as_bytes()[i] - b'a') as usize;
            pairs[i1].push(i2);
            pairs[i2].push(i1);
        }

        let replace = (0..26)
            .map(|i| {
                let mut min = i;
                let mut stack = vec![i];
                let mut seen = vec![false; 26];
                while let Some(j) = stack.pop() {
                    if seen[j] {
                        continue;
                    }
                    min = min.min(j);
                    seen[j] = true;
                    for k in &pairs[j] {
                        if !seen[*k] {
                            stack.push(*k);
                        }
                    }
                }

                min as u8
            })
            .collect::<Vec<_>>();

        base_str
            .as_bytes()
            .iter()
            .map(|b| (replace[(b - b'a') as usize] + b'a') as char)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1061() {
        assert_eq!(
            Solution::smallest_equivalent_string(
                "parker".to_string(),
                "morris".to_string(),
                "parser".to_string()
            ),
            "makkek".to_string()
        );
        assert_eq!(
            Solution::smallest_equivalent_string(
                "hello".to_string(),
                "world".to_string(),
                "hold".to_string()
            ),
            "hdld".to_string()
        );
        assert_eq!(
            Solution::smallest_equivalent_string(
                "leetcode".to_string(),
                "programs".to_string(),
                "sourcecode".to_string()
            ),
            "aauaaaaada".to_string()
        );
    }
}
