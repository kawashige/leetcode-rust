pub struct Solution {}

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut appeared = 0;
        let mut count = 1;

        for b in s.as_bytes() {
            if appeared & 1 << b - b'a' != 0 {
                count += 1;
                appeared = 0;
            }
            appeared |= 1 << b - b'a';
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2405() {
        assert_eq!(Solution::partition_string("abacaba".to_string()), 4);
        assert_eq!(Solution::partition_string("ssssss".to_string()), 6);
    }
}
