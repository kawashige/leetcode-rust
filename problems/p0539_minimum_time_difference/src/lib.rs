pub struct Solution {}

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut minutes = time_points
            .iter()
            .map(|p| {
                let time = p
                    .split(":")
                    .map(|t| t.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                time[0] * 60 + time[1]
            })
            .collect::<Vec<usize>>();
        minutes.sort_unstable();

        let min = (1..minutes.len())
            .map(|i| minutes[i] - minutes[i - 1])
            .min()
            .unwrap();
        std::cmp::min(min, minutes[0] + 24 * 60 - minutes[minutes.len() - 1]) as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0539() {
        assert_eq!(
            1,
            Solution::find_min_difference(vec!["23:59".to_string(), "00:00".to_string()])
        );
        assert_eq!(
            0,
            Solution::find_min_difference(vec![
                "00:00".to_string(),
                "23:59".to_string(),
                "00:00".to_string()
            ])
        );
    }
}
