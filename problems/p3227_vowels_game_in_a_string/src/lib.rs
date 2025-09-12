pub struct Solution {}

impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        let count = s.as_bytes().iter().fold(0, |count, b| {
            if [b'a', b'e', b'i', b'o', b'u'].contains(b) {
                count + 1
            } else {
                count
            }
        });
        count != 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3227() {
        assert!(Solution::does_alice_win("leetcoder".to_string()));
        assert!(!Solution::does_alice_win("bbcd".to_string()));
    }
}
