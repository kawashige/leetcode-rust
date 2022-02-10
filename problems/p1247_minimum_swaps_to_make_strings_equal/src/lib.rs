pub struct Solution {}

impl Solution {
    pub fn minimum_swap(s1: String, s2: String) -> i32 {
        let (x, y) =
            s1.as_bytes()
                .iter()
                .zip(s2.as_bytes().iter())
                .fold((0, 0), |(x, y), (b1, b2)| match (b1, b2) {
                    (b'x', b'y') => (x + 1, y),
                    (b'y', b'x') => (x, y + 1),
                    _ => (x, y),
                });

        if x % 2 + y % 2 == 1 {
            -1
        } else {
            x / 2 + y / 2 + x % 2 + y % 2
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1247() {
        assert_eq!(
            Solution::minimum_swap("xx".to_string(), "yy".to_string()),
            1
        );
        assert_eq!(
            Solution::minimum_swap("xy".to_string(), "yx".to_string()),
            2
        );
        assert_eq!(
            Solution::minimum_swap("xx".to_string(), "xy".to_string()),
            -1
        );
    }
}
