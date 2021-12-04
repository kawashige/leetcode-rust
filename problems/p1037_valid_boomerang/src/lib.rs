pub struct Solution {}

impl Solution {
    pub fn is_boomerang(mut points: Vec<Vec<i32>>) -> bool {
        points.sort_unstable();

        !(points[0][0] == points[1][0] && points[0][1] == points[1][1])
            && !(points[1][0] == points[2][0] && points[1][1] == points[2][1])
            && (points[1][1] - points[0][1]) * (points[2][0] - points[1][0])
                != (points[2][1] - points[1][1]) * (points[1][0] - points[0][0])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1037() {
        assert!(Solution::is_boomerang(vec![
            vec![1, 1],
            vec![2, 3],
            vec![3, 2]
        ]));
        assert!(!Solution::is_boomerang(vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 3]
        ]));
    }
}
