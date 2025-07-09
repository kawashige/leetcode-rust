pub struct Solution {}

impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let mut free_times = Vec::new();
        free_times.push(start_time[0]);
        for i in 1..start_time.len() {
            free_times.push(start_time[i] - end_time[i - 1]);
        }
        free_times.push(event_time - end_time[end_time.len() - 1]);

        let mut tmp_sum = free_times[0..(k as usize + 1).min(free_times.len())]
            .iter()
            .sum::<i32>();
        let mut result = tmp_sum;
        for i in k as usize + 1..free_times.len() {
            tmp_sum += free_times[i] - free_times[i - k as usize - 1];
            result = result.max(tmp_sum);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3439() {
        assert_eq!(Solution::max_free_time(5, 1, vec![1, 3], vec![2, 5]), 2);
        assert_eq!(
            Solution::max_free_time(10, 1, vec![0, 2, 9], vec![1, 4, 10]),
            6
        );
        assert_eq!(
            Solution::max_free_time(5, 2, vec![0, 1, 2, 3, 4], vec![1, 2, 3, 4, 5]),
            0
        );
    }
}
