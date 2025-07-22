pub struct Solution {}

impl Solution {
    pub fn winning_player(x: i32, y: i32) -> String {
        let mut x = x;
        let mut y = y;
        for i in 0.. {
            x -= 1;
            y -= 4;
            if x < 0 || y < 0 {
                return if i % 2 == 0 {
                    "Bob".to_string()
                } else {
                    "Alice".to_string()
                };
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3222() {
        assert_eq!(Solution::winning_player(2, 7), "Alice".to_string());
        assert_eq!(Solution::winning_player(4, 11), "Bob".to_string());
    }
}
