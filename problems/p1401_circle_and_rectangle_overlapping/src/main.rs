pub struct Solution {}

impl Solution {
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        if (x1..=x2).contains(&x_center) && (y1..=y2).contains(&y_center) {
            return true;
        }

        let radius_square = radius as i64 * radius as i64;

        for x in x1..=x2 {
            if (x - x_center) as i64 * (x - x_center) as i64
                + (y1 - y_center) as i64 * (y1 - y_center) as i64
                <= radius_square
            {
                return true;
            }
        }
        for x in x1..=x2 {
            if (x - x_center) as i64 * (x - x_center) as i64
                + (y2 - y_center) as i64 * (y2 - y_center) as i64
                <= radius_square
            {
                return true;
            }
        }
        for y in y1..=y2 {
            if (x1 - x_center) as i64 * (x1 - x_center) as i64
                + (y - y_center) as i64 * (y - y_center) as i64
                <= radius_square
            {
                return true;
            }
        }
        for y in y1..=y2 {
            if (x2 - x_center) as i64 * (x2 - x_center) as i64
                + (y - y_center) as i64 * (y - y_center) as i64
                <= radius_square
            {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1401() {
        assert!(Solution::check_overlap(1, 0, 0, 1, -1, 3, 1));
        assert!(!Solution::check_overlap(1, 1, 1, 1, -3, 2, -1));
        assert!(Solution::check_overlap(1, 0, 0, -1, 0, 0, 1));
    }
}

fn main() {}
