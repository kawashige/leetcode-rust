pub struct Solution {}

impl Solution {
    pub fn count_rectangles(rectangles: Vec<Vec<i32>>, points: Vec<Vec<i32>>) -> Vec<i32> {
        let mut counts = vec![0; 101];
        let mut rectangles = rectangles;
        let mut points = points.into_iter().zip(0..).collect::<Vec<_>>();
        rectangles.sort_unstable_by_key(|r| -r[0]);
        points.sort_unstable_by_key(|(p, _)| -p[0]);

        let mut i = 0;
        let mut result = vec![0; points.len()];
        for j in 0..points.len() {
            while i < rectangles.len() && points[j].0[0] <= rectangles[i][0] {
                counts[rectangles[i][1] as usize] += 1;
                i += 1;
            }
            result[points[j].1] = (points[j].0[1] as usize..counts.len())
                .map(|i| counts[i])
                .sum();
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3250() {
        assert_eq!(
            Solution::count_rectangles(
                vec![vec![1, 2], vec![2, 3], vec![2, 5]],
                vec![vec![2, 1], vec![1, 4]]
            ),
            vec![2, 1]
        );
        assert_eq!(
            Solution::count_rectangles(
                vec![vec![1, 1], vec![2, 2], vec![3, 3]],
                vec![vec![1, 3], vec![1, 1]]
            ),
            vec![1, 3]
        );
    }
}
