pub struct Solution {}

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let mut angle = ((hour % 12) * 60 + minutes - minutes * 12).abs();
        if angle > 360 {
            angle = 720 - angle;
        }
        angle as f64 / 2.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1344() {
        assert_eq!(Solution::angle_clock(1, 57), 76.50000);
        assert_eq!(Solution::angle_clock(12, 30), 165.0);
        assert_eq!(Solution::angle_clock(3, 30), 75.0);
        assert_eq!(Solution::angle_clock(3, 15), 7.5);
    }
}
