pub struct Solution {}

impl Solution {
    pub fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
        let (_, x, y) = (0..=50 + radius)
            .map(|x| {
                (0..=50 + radius)
                    .map(|y| {
                        let p = towers
                            .iter()
                            .map(|t| {
                                let d_square = (t[0] - x).pow(2) + (t[1] - y).pow(2);
                                if d_square <= radius.pow(2) {
                                    (t[2] as f64 / (1.0 + (d_square as f64).sqrt())) as i32
                                } else {
                                    0
                                }
                            })
                            .sum::<i32>();
                        (p, -x, -y)
                    })
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap();
        vec![-x, -y]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1620() {
        assert_eq!(
            Solution::best_coordinate(
                vec![
                    vec![31, 13, 33],
                    vec![24, 45, 38],
                    vec![28, 32, 23],
                    vec![7, 23, 22],
                    vec![41, 50, 33],
                    vec![47, 21, 3],
                    vec![3, 33, 39],
                    vec![11, 38, 5],
                    vec![26, 20, 28],
                    vec![48, 39, 16],
                    vec![34, 29, 25]
                ],
                21
            ),
            vec![24, 45]
        );
        assert_eq!(
            Solution::best_coordinate(vec![vec![1, 2, 5], vec![2, 1, 7], vec![3, 1, 9]], 2),
            vec![2, 1]
        );
        assert_eq!(
            Solution::best_coordinate(vec![vec![23, 11, 21]], 9),
            vec![23, 11]
        );
        assert_eq!(
            Solution::best_coordinate(vec![vec![1, 2, 13], vec![2, 1, 7], vec![0, 1, 9]], 2),
            vec![1, 2]
        );
    }
}
