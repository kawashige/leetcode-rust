pub struct Solution {}

impl Solution {
    pub fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
        (arrival_time + delayed_time) % 24
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2651() {
        assert_eq!(Solution::find_delayed_arrival_time(15, 5), 20);
        assert_eq!(Solution::find_delayed_arrival_time(13, 11), 0);
    }
}
