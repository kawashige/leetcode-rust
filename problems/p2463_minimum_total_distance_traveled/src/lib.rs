pub struct Solution {}

impl Solution {
    pub fn minimum_total_distance(robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {
        let mut robot = robot;
        let mut factory = factory;
        robot.sort_unstable();
        factory.sort_unstable();

        let mut dp = vec![vec![std::i64::MAX; factory.len() + 1]; robot.len() + 1];
        dp[0][0] = 0;

        for i in 0..robot.len() {
            for j in 0..factory.len() {
                let mut d = 0;
                for k in 0..(factory[j][1] as usize).min(i + 1) {
                    d += (robot[i - k] - factory[j][0]).abs() as i64;
                    for l in 0..=j {
                        if dp[i - k][j - l] != std::i64::MAX {
                            dp[i + 1][j + 1] = dp[i + 1][j + 1].min(dp[i - k][j - l] + d);
                        }
                    }
                }
            }
        }
        println!("dp: {:?}", dp);
        *dp.last().unwrap().into_iter().min().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2463() {
        assert_eq!(
            Solution::minimum_total_distance(
                vec![9, 11, 99, 101],
                vec![
                    vec![10, 1],
                    vec![7, 1],
                    vec![14, 1],
                    vec![100, 1],
                    vec![96, 1],
                    vec![103, 1]
                ]
            ),
            6
        );
        assert_eq!(
            Solution::minimum_total_distance(vec![0, 4, 6], vec![vec![2, 2], vec![6, 2]]),
            4
        );
        assert_eq!(
            Solution::minimum_total_distance(vec![1, -1], vec![vec![-2, 1], vec![2, 1]]),
            2
        );
    }
}
