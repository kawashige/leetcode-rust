pub struct Solution {}

impl Solution {
    pub fn get_smallest_string(s: String) -> String {
        let mut chars = s.chars().collect::<Vec<_>>();
        for i in 0..chars.len() - 1 {
            if (chars[i] as u8 - b'0') % 2 == (chars[i + 1] as u8 - b'0') % 2
                && chars[i] > chars[i + 1]
            {
                chars.swap(i, i + 1);
                break;
            }
        }
        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3216() {
        assert_eq!(
            Solution::get_smallest_string("45320".to_string()),
            "43520".to_string()
        );
        assert_eq!(
            Solution::get_smallest_string("001".to_string()),
            "001".to_string()
        );
    }
}
