pub struct Solution {}

impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let mut moves = 0;
        let mut i = 0;

        while i < s.len() {
            if s.as_bytes()[i] == b'X' {
                moves += 1;
                i += 3;
            } else {
                i += 1;
            }
        }

        moves
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2027() {
        assert_eq!(Solution::minimum_moves("XXX".to_string()), 1);
        assert_eq!(Solution::minimum_moves("XX0X".to_string()), 2);
        assert_eq!(Solution::minimum_moves("0000".to_string()), 0);
    }
}
