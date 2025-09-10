pub struct Solution {}

impl Solution {
    pub fn button_with_longest_time(events: Vec<Vec<i32>>) -> i32 {
        let mut button = events[0][0];
        let mut time = events[0][1];
        for i in 1..events.len() {
            if time <= events[i][1] - events[i - 1][1] {
                if time == events[i][1] - events[i - 1][1] {
                    button = events[i][0].min(button);
                } else {
                    button = events[i][0];
                }
                time = events[i][1] - events[i - 1][1];
            }
        }
        button
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3386() {
        assert_eq!(
            Solution::button_with_longest_time(vec![
                vec![1, 2],
                vec![2, 5],
                vec![3, 9],
                vec![1, 15]
            ]),
            1
        );
        assert_eq!(
            Solution::button_with_longest_time(vec![vec![10, 5], vec![1, 7]]),
            10
        );
    }
}
