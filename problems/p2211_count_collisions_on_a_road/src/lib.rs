pub struct Solution {}

impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let mut collisions = 0;

        let mut right = 0;
        for b in directions.as_bytes() {
            if b == &b'R' {
                right += 1;
            } else {
                collisions += right;
                right = 0;
            }
        }

        let mut left = 0;
        for b in directions.as_bytes().iter().rev() {
            if b == &b'L' {
                left += 1;
            } else {
                collisions += left;
                left = 0;
            }
        }

        collisions
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2211() {
        assert_eq!(Solution::count_collisions("RLRSLL".to_string()), 5);
        assert_eq!(Solution::count_collisions("LLRR".to_string()), 0);
    }
}
