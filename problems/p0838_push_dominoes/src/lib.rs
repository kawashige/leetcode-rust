pub struct Solution {}

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let bytes = dominoes.as_bytes();
        let mut left = vec![std::usize::MAX; dominoes.len()];
        let mut right = vec![std::usize::MAX; dominoes.len()];

        for i in 0..bytes.len() {
            if bytes[i] == b'.' && i > 0 && right[i - 1] != std::usize::MAX {
                right[i] = right[i - 1] + 1;
            } else if bytes[i] == b'R' {
                right[i] = 0;
            }

            if bytes[bytes.len() - 1 - i] == b'.'
                && i > 0
                && left[bytes.len() - i] != std::usize::MAX
            {
                left[bytes.len() - 1 - i] = left[bytes.len() - i] + 1;
            } else if bytes[bytes.len() - 1 - i] == b'L' {
                left[bytes.len() - 1 - i] = 0;
            }
        }

        dominoes
            .chars()
            .enumerate()
            .map(|(i, c)| match c {
                '.' if right[i] < left[i] => 'R',
                '.' if left[i] < right[i] => 'L',
                _ => c,
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0838() {
        assert_eq!(Solution::push_dominoes("".to_string()), "".to_string());

        assert_eq!(
            Solution::push_dominoes(".L.R...LR..L..".to_string()),
            "LL.RR.LLRRLL..".to_string()
        );

        assert_eq!(
            Solution::push_dominoes("RR.L".to_string()),
            "RR.L".to_string()
        );
    }
}
