pub struct Solution {}

impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        (0..32).fold(0, |count, i| {
            count
                + match (a >> i & 1, b >> i & 1, c >> i & 1) {
                    (1, 0, 0) | (0, 1, 0) | (0, 0, 1) => 1,
                    (1, 1, 0) => 2,
                    _ => 0,
                }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1318() {
        assert_eq!(Solution::min_flips(2, 6, 5), 3);
        assert_eq!(Solution::min_flips(4, 2, 7), 1);
        assert_eq!(Solution::min_flips(1, 2, 3), 0);
    }
}
