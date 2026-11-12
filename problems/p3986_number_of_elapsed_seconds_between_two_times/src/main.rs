pub struct Solution {}

impl Solution {
    pub fn seconds_between_times(start_time: String, end_time: String) -> i32 {
        (end_time[..2].parse::<i32>().unwrap() - start_time[..2].parse::<i32>().unwrap()) * 60 * 60
            + (end_time[3..5].parse::<i32>().unwrap() - start_time[3..5].parse::<i32>().unwrap())
                * 60
            + (end_time[6..].parse::<i32>().unwrap() - start_time[6..].parse::<i32>().unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3986() {
        assert_eq!(
            Solution::seconds_between_times("01:00:00".to_string(), "01:00:25".to_string()),
            25
        );
        assert_eq!(
            Solution::seconds_between_times("12:34:56".to_string(), "13:00:00".to_string()),
            1504
        );
    }
}

fn main() {}
