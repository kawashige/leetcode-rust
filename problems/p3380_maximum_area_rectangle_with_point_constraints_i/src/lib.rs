pub struct Solution {}

impl Solution {
    pub fn max_rectangle_area(points: Vec<Vec<i32>>) -> i32 {
        let mut coodinates = vec![vec![101; 101]; 101];
        for i in 0..points.len() {
            coodinates[points[i][0] as usize][points[i][1] as usize] = i;
        }

        let mut max_area = -1;
        for i in 0..points.len() {
            for j in 0..points.len() {
                if points[i][0] == points[j][0] || points[i][1] == points[j][1] {
                    continue;
                }
                let k = coodinates[points[i][0] as usize][points[j][1] as usize];
                let l = coodinates[points[j][0] as usize][points[i][1] as usize];
                if k != 101
                    && l != 101
                    && (0..points.len()).all(|x| {
                        [i, j, k, l].contains(&x)
                            || !(points[i][0].min(points[j][0])..=points[i][0].max(points[j][0]))
                                .contains(&points[x][0])
                            || !(points[i][1].min(points[j][1])..=points[i][1].max(points[j][1]))
                                .contains(&points[x][1])
                    })
                {
                    let area = (points[i][0].max(points[j][0]) - points[i][0].min(points[j][0]))
                        * (points[i][1].max(points[j][1]) - points[i][1].min(points[j][1]));
                    max_area = max_area.max(area);
                }
            }
        }
        max_area
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3380() {
        assert_eq!(
            Solution::max_rectangle_area(vec![vec![1, 1], vec![1, 3], vec![3, 1], vec![3, 3]]),
            4
        );
        assert_eq!(
            Solution::max_rectangle_area(vec![
                vec![1, 1],
                vec![1, 3],
                vec![3, 1],
                vec![3, 3],
                vec![2, 2]
            ]),
            -1
        );
        assert_eq!(
            Solution::max_rectangle_area(vec![
                vec![1, 1],
                vec![1, 3],
                vec![3, 1],
                vec![3, 3],
                vec![1, 2],
                vec![3, 2]
            ]),
            2
        );
    }
}
