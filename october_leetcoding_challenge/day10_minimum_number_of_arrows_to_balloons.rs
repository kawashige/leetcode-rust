pub struct Solution {}

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        if points.is_empty() {
            return 0;
        }
        let mut points = points;
        points.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut result = 1;
        let mut current = points[0][1];
        for i in 1..points.len() {
            if points[i][0] <= current {
                continue;
            }
            result += 1;
            current = points[i][1]
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day10() {
        assert_eq!(
            2,
            Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]])
        );
        assert_eq!(
            4,
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]])
        );
        assert_eq!(
            2,
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]])
        );
        assert_eq!(1, Solution::find_min_arrow_shots(vec![vec![1, 2]]));
        assert_eq!(1, Solution::find_min_arrow_shots(vec![vec![1, 2147483647]]));
        assert_eq!(
            1,
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![1, 2]])
        );
        assert_eq!(
            3,
            Solution::find_min_arrow_shots(vec![
                vec![2, 3],
                vec![7, 15],
                vec![5, 12],
                vec![4, 5],
                vec![8, 13],
                vec![9, 16],
                vec![5, 8],
                vec![8, 16],
                vec![3, 4],
                vec![8, 17]
            ])
        );
        assert_eq!(
            4,
            Solution::find_min_arrow_shots(vec![
                vec![4289383, 51220269],
                vec![81692777, 96329692],
                vec![57747793, 81986128],
                vec![19885386, 69645878],
                vec![96516649, 186158070],
                vec![25202362, 75692389],
                vec![83368690, 85888749],
                vec![44897763, 112411689],
                vec![65180540, 105563966],
                vec![4089172, 7544908]
            ])
        );
        assert_eq!(
            2,
            Solution::find_min_arrow_shots(vec![vec![-1, 1], vec![0, 1], vec![2, 3], vec![1, 2]])
        );
    }
}
