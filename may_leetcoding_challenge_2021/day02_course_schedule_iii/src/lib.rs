pub struct Solution {}

impl Solution {
    pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
        courses.sort_unstable_by(|a, b| a[1].cmp(&b[1]));

        let mut dp = vec![0; courses.last().unwrap()[1] as usize + 1];

        for course in courses {
            if course[0] > course[1] {
                continue;
            }
            for j in (1..=(course[1] - course[0])).rev() {
                if dp[j as usize] > 0 {
                    dp[(j + course[0]) as usize] =
                        std::cmp::max(dp[j as usize] + 1, dp[(j + course[0]) as usize]);
                }
            }
            if dp[course[0] as usize] == 0 {
                dp[course[0] as usize] = 1;
            }
        }

        dp.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day02() {
        assert_eq!(
            Solution::schedule_course(vec![vec![1, 19], vec![2, 2], vec![1, 17]]),
            3
        );
        assert_eq!(
            Solution::schedule_course(vec![
                vec![7, 16],
                vec![2, 3],
                vec![3, 12],
                vec![3, 14],
                vec![10, 19],
                vec![10, 16],
                vec![6, 8],
                vec![6, 11],
                vec![3, 13],
                vec![6, 16]
            ]),
            4
        );
        assert_eq!(
            Solution::schedule_course(vec![
                vec![100, 200],
                vec![200, 1300],
                vec![1000, 1250],
                vec![2000, 3200]
            ]),
            3
        );
        assert_eq!(Solution::schedule_course(vec![vec![1, 2]]), 1);
        assert_eq!(Solution::schedule_course(vec![vec![3, 2], vec![4, 3]]), 0);
    }
}
