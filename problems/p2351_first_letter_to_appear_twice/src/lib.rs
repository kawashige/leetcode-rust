pub struct Solution {}

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut appeared = 0;
        for i in 0..s.len() {
            if appeared & 1 << (s.as_bytes()[i] - b'a') as usize != 0 {
                return s.as_bytes()[i] as char;
            }
            appeared |= 1 << (s.as_bytes()[i] - b'a') as usize;
        }

        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2351() {
        assert_eq!(Solution::repeated_character("abccbaacz".to_string()), 'c');
        assert_eq!(Solution::repeated_character("abcdd".to_string()), 'd');
    }
}
