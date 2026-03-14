pub struct Solution {}

impl Solution {
    pub fn concat_hex36(n: i32) -> String {
        let values = ('0'..='9').chain('A'..='Z').collect::<Vec<_>>();
        let mut chars = Vec::new();

        let mut n3 = n * n * n;
        while 0 < n3 {
            chars.push(values[(n3 % 36) as usize]);
            n3 /= 36;
        }

        let mut n2 = n * n;
        while 0 < n2 {
            chars.push(values[(n2 % 16) as usize]);
            n2 /= 16;
        }

        chars.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3602() {
        assert_eq!(Solution::concat_hex36(13), "A91P1".to_string());
        assert_eq!(Solution::concat_hex36(36), "5101000".to_string());
    }
}
