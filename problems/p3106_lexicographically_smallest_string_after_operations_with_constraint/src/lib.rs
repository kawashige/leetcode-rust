pub struct Solution {}

impl Solution {
    pub fn get_smallest_string(s: String, k: i32) -> String {
        let mut result = String::new();
        let mut k = k;

        for i in 0..s.len() {
            if k == 0 || s.as_bytes()[i] == b'a' {
                result.push(s.as_bytes()[i] as char);
            } else if b'z' + 1 - s.as_bytes()[i] <= s.as_bytes()[i] - b'a'
                && (b'z' + 1 - s.as_bytes()[i]) as i32 <= k
            {
                result.push('a');
                k -= (b'z' + 1 - s.as_bytes()[i]) as i32;
            } else {
                let d = k.min((s.as_bytes()[i] - b'a') as i32);
                result.push((s.as_bytes()[i] - d as u8) as char);
                k -= d;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3106() {
        assert_eq!(
            Solution::get_smallest_string("zbbz".to_string(), 3),
            "aaaz".to_string()
        );
        assert_eq!(
            Solution::get_smallest_string("xaxcd".to_string(), 4),
            "aawcd".to_string()
        );
        assert_eq!(
            Solution::get_smallest_string("lol".to_string(), 0),
            "lol".to_string()
        );
    }
}
