pub struct Solution {}

impl Solution {
    pub fn maximum_binary_string(binary: String) -> String {
        let mut zero_index = Vec::with_capacity(binary.len());
        for i in (0..binary.len()).rev() {
            if binary.as_bytes()[i] == b'0' {
                zero_index.push(i);
            }
        }

        let mut chars = binary.chars().collect::<Vec<_>>();

        for i in 0..chars.len() - 1 {
            if zero_index.last() == Some(&i) {
                zero_index.pop();
            }
            match (chars[i], chars[i + 1]) {
                ('0', '0') => chars[i] = '1',
                ('0', '1') if !zero_index.is_empty() => {
                    chars[i] = '1';
                    chars[i + 1] = '0';
                    chars[zero_index.pop().unwrap()] = '1';
                }
                _ => {}
            }
        }

        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1702() {
        assert_eq!(
            Solution::maximum_binary_string("000110".to_string()),
            "111011".to_string()
        );
        assert_eq!(
            Solution::maximum_binary_string("01".to_string()),
            "01".to_string()
        );
    }
}
