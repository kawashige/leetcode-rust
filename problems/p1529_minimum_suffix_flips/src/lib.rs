pub struct Solution {}

impl Solution {
    pub fn min_flips(target: String) -> i32 {
        let mut current = 0;
        let mut flip = 0;

        for b in target.as_bytes().iter() {
            if b - b'0' != current {
                flip += 1;
                current = 1 - current;
            }
        }

        flip
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1529() {
        assert_eq!(Solution::min_flips("10111".to_string()), 3);
        assert_eq!(Solution::min_flips("101".to_string()), 3);
        assert_eq!(Solution::min_flips("00000".to_string()), 0);
    }
}
