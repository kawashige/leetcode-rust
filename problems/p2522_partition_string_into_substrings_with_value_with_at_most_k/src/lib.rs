pub struct Solution {}

impl Solution {
    pub fn minimum_partition(s: String, k: i32) -> i32 {
        let k = k as i64;
        let mut val = 0;
        let mut count = 1;

        for i in 0..s.len() {
            let d = (s.as_bytes()[i] - b'0') as i64;
            if k < d {
                return -1;
            }
            if val * 10 + d <= k {
                val = val * 10 + d;
            } else {
                val = d;
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2522() {
        assert_eq!(Solution::minimum_partition("165462".to_string(), 60), 4);
        assert_eq!(Solution::minimum_partition("238182".to_string(), 5), -1);
    }
}
