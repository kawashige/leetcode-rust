pub struct Solution {}

impl Solution {
    pub fn seconds_to_remove_occurrences(s: String) -> i32 {
        let mut seconds = 0;
        let mut chars = s.chars().collect::<Vec<_>>();
        let mut changed = true;

        while changed {
            changed = false;
            let mut i = 0;
            while i < chars.len() - 1 {
                if chars[i] == '0' && chars[i + 1] == '1' {
                    chars[i] = '1';
                    chars[i + 1] = '0';
                    changed = true;
                    i += 1;
                }
                i += 1;
            }
            if changed {
                seconds += 1;
            }
        }

        seconds
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2380() {
        assert_eq!(
            Solution::seconds_to_remove_occurrences("0110101".to_string()),
            4
        );
        assert_eq!(
            Solution::seconds_to_remove_occurrences("11100".to_string()),
            0
        );
    }
}
