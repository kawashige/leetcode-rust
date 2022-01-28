pub struct Solution {}

impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        let mut max_len = 0;
        let mut i = 0;
        let mut cost = 0;

        for j in 0..s_bytes.len() {
            cost += (s_bytes[j] as i32 - t_bytes[j] as i32).abs();
            while max_cost < cost {
                cost -= (s_bytes[i] as i32 - t_bytes[i] as i32).abs();
                i += 1;
            }

            max_len = max_len.max(j as i32 - i as i32 + 1);
        }

        max_len
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1208() {
        assert_eq!(
            Solution::equal_substring("abcd".to_string(), "bcdf".to_string(), 3),
            3
        );
        assert_eq!(
            Solution::equal_substring("abcd".to_string(), "cdef".to_string(), 3),
            1
        );
        assert_eq!(
            Solution::equal_substring("abcd".to_string(), "acde".to_string(), 0),
            1
        );
    }
}
