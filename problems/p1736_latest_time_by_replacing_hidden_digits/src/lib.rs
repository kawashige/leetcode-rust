pub struct Solution {}

impl Solution {
    pub fn maximum_time(time: String) -> String {
        let h1 = if time.as_bytes()[0] == b'?' {
            if (b'4'..=b'9').contains(&time.as_bytes()[1]) {
                b'1'
            } else {
                b'2'
            }
        } else {
            time.as_bytes()[0]
        };
        let h2 = if time.as_bytes()[1] == b'?' {
            if h1 == b'2' {
                b'3'
            } else {
                b'9'
            }
        } else {
            time.as_bytes()[1]
        };
        let m1 = if time.as_bytes()[3] == b'?' {
            b'5'
        } else {
            time.as_bytes()[3]
        };
        let m2 = if time.as_bytes()[4] == b'?' {
            b'9'
        } else {
            time.as_bytes()[4]
        };
        format!("{}{}:{}{}", h1 as char, h2 as char, m1 as char, m2 as char)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1736() {
        assert_eq!(
            Solution::maximum_time("??:3?".to_string()),
            "23:39".to_string()
        );
        assert_eq!(
            Solution::maximum_time("?4:03".to_string()),
            "14:03".to_string()
        );
        assert_eq!(
            Solution::maximum_time("2?:?0".to_string()),
            "23:50".to_string()
        );
        assert_eq!(
            Solution::maximum_time("0?:3?".to_string()),
            "09:39".to_string()
        );
        assert_eq!(
            Solution::maximum_time("1?:22".to_string()),
            "19:22".to_string()
        );
    }
}
