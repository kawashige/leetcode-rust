pub struct Solution {}

impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        let mut max_l = 0;
        for i in 0..bottom_left.len() {
            for j in 0..i {
                if top_right[i][0] < bottom_left[j][0]
                    || top_right[j][0] < bottom_left[i][0]
                    || top_right[i][1] < bottom_left[j][1]
                    || top_right[j][1] < bottom_left[i][1]
                {
                    continue;
                }

                let l = (top_right[i][0].min(top_right[j][0])
                    - bottom_left[i][0].max(bottom_left[j][0]))
                .min(
                    top_right[i][1].min(top_right[j][1]) - bottom_left[i][1].max(bottom_left[j][1]),
                );
                max_l = max_l.max(l);
            }
        }
        max_l as i64 * max_l as i64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3047() {
        assert_eq!(
            Solution::largest_square_area(
                vec![vec![1, 1], vec![2, 2], vec![3, 1]],
                vec![vec![3, 3], vec![4, 4], vec![6, 6]]
            ),
            1
        );
        assert_eq!(
            Solution::largest_square_area(
                vec![vec![1, 1], vec![1, 3], vec![1, 5]],
                vec![vec![5, 5], vec![5, 7], vec![5, 9]]
            ),
            4
        );
        assert_eq!(
            Solution::largest_square_area(
                vec![vec![1, 1], vec![2, 2], vec![1, 2]],
                vec![vec![3, 3], vec![4, 4], vec![3, 4]]
            ),
            1
        );
        assert_eq!(
            Solution::largest_square_area(
                vec![vec![1, 1], vec![3, 3], vec![3, 1]],
                vec![vec![2, 2], vec![4, 4], vec![4, 2]]
            ),
            0
        );
    }
}
