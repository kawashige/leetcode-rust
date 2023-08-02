pub struct Solution {}

impl Solution {
    pub fn minimum_buckets(hamsters: String) -> i32 {
        let mut last_bucket = -2;
        let mut count = 0;

        for i in 0..hamsters.len() {
            if hamsters.as_bytes()[i] == b'.' {
                continue;
            }
            if last_bucket == i as i32 - 1 {
                // nothing to do
            } else if i + 1 < hamsters.len() && hamsters.as_bytes()[i + 1] == b'.' {
                last_bucket = i as i32 + 1;
                count += 1;
            } else if 0 < i && hamsters.as_bytes()[i - 1] == b'.' {
                last_bucket = i as i32 - 1;
                count += 1;
            } else {
                return -1;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2086() {
        assert_eq!(Solution::minimum_buckets("H..H".to_string()), 2);
        assert_eq!(Solution::minimum_buckets(".H.H.".to_string()), 1);
        assert_eq!(Solution::minimum_buckets(".HHH.".to_string()), -1);
    }
}
