pub struct Solution {}

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut remains = [a, b, c];
        let mut chars = Vec::new();
        let mut changed = true;

        while changed {
            changed = false;

            let mut indices = vec![0, 1, 2];
            indices.sort_unstable_by_key(|i| -remains[*i]);

            for i in indices {
                if remains[i] > 0
                    && (chars.len() < 2
                        || (chars[chars.len() - 1] != i || chars[chars.len() - 2] != i))
                {
                    remains[i] -= 1;
                    chars.push(i);
                    changed = true;
                    break;
                }
            }
        }

        chars
            .into_iter()
            .map(|i| (b'a' + i as u8) as char)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1405() {
        assert_eq!(
            Solution::longest_diverse_string(1, 1, 7),
            "ccaccbcc".to_string()
        );
        assert_eq!(
            Solution::longest_diverse_string(7, 1, 0),
            "aabaa".to_string()
        );
    }
}
