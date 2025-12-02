pub struct Solution {}

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        const M: usize = 1_000_000_007;

        let mut points = points;
        points.sort_unstable_by_key(|p| p[1]);

        let mut sum = 0;
        let mut count = 1;
        let mut result = 0;

        for i in 1..points.len() {
            if points[i][1] != points[i - 1][1] {
                let tmp_sum = count * (count - 1) / 2;
                result += tmp_sum * sum;
                result %= M;
                sum += tmp_sum;
                sum %= M;
                count = 0;
            }
            count += 1;
        }
        let tmp_sum = count * (count - 1) / 2;
        result += tmp_sum * sum;
        result %= M;
        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3623() {
        assert_eq!(
            Solution::count_trapezoids(vec![
                vec![1, 0],
                vec![2, 0],
                vec![3, 0],
                vec![2, 2],
                vec![3, 2]
            ]),
            3
        );
        assert_eq!(
            Solution::count_trapezoids(vec![vec![0, 0], vec![1, 0], vec![0, 1], vec![2, 1]]),
            1
        );
    }
}
