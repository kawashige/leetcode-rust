pub struct Solution {}

impl Solution {
    pub fn largest_triangle_area(mut points: Vec<Vec<i32>>) -> f64 {
        points.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));

        let mut max = 0.0;
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                for k in (j + 1)..points.len() {
                    if (points[i][0] == points[j][0] && points[i][0] == points[k][0])
                        || (points[i][1] == points[j][1] && points[i][1] == points[k][1])
                    {
                        continue;
                    }

                    let area = ((points[j][0] - points[i][0]) * (points[k][1] - points[i][1])
                        - (points[k][0] - points[i][0]) * (points[j][1] - points[i][1]))
                        .abs() as f64
                        * 0.5;
                    if area > max {
                        max = area;
                    }
                }
            }
        }
        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0812() {
        assert_eq!(
            Solution::largest_triangle_area(vec![
                vec![0, 0],
                vec![0, 1],
                vec![1, 0],
                vec![0, 2],
                vec![2, 0]
            ]),
            2.0
        );
    }
}
