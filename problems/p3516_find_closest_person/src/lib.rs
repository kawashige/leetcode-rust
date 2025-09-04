pub struct Solution {}

impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        match ((x - z).abs() - (y - z).abs()).signum() {
            0 => 0,
            -1 => 1,
            1 => 2,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3516() {
        assert_eq!(Solution::find_closest(2, 7, 4), 1);
        assert_eq!(Solution::find_closest(2, 5, 6), 2);
        assert_eq!(Solution::find_closest(1, 5, 3), 0);
    }
}
