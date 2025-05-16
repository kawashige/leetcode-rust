pub struct Solution {}

impl Solution {
    pub fn find_latest_time(s: String) -> String {
        let mut result = String::new();

        for i in 0..s.len() {
            if s.as_bytes()[i] == b'?' {
                result.push(match i {
                    0 if [b'0', b'1', b'?'].contains(&s.as_bytes()[1]) => '1',
                    0 => '0',
                    1 if s.as_bytes()[0] == b'0' => '9',
                    1 => '1',
                    3 => '5',
                    _ => '9',
                });
            } else {
                result.push(s.as_bytes()[i] as char);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3114() {
        assert_eq!(
            Solution::find_latest_time("1?:?4".to_string()),
            "11:54".to_string()
        );
        assert_eq!(
            Solution::find_latest_time("0?:5?".to_string()),
            "09:59".to_string()
        );
    }
}
