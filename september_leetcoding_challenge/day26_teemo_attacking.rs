pub struct Solution {}

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        if time_series.len() == 0 {
            return 0;
        }

        let mut result = 0;
        let mut start = time_series[0];
        let mut end = start + duration - 1;
        for i in 1..time_series.len() {
            if end < time_series[i] {
                result += end - start + 1;
            } else {
                result += time_series[i] - 1 - start + 1;
            }
            start = time_series[i];
            end = start + duration - 1;
        }
        result + end - start + 1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day26() {
        assert_eq!(4, Solution::find_poisoned_duration(vec![1, 4], 2));
        assert_eq!(3, Solution::find_poisoned_duration(vec![1, 2], 2));
        assert_eq!(5, Solution::find_poisoned_duration(vec![1, 2, 3], 3));
        assert_eq!(8, Solution::find_poisoned_duration(vec![1, 2, 3, 12], 3));
    }
}
