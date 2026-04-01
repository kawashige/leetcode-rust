pub struct Solution {}

impl Solution {
    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        let mut result = std::i32::MAX;

        let mut t1 = std::i32::MAX;
        for i in 0..land_start_time.len() {
            t1 = t1.min(land_start_time[i] + land_duration[i]);
        }

        let mut t2 = std::i32::MAX;
        for i in 0..water_start_time.len() {
            t2 = t2.min(water_start_time[i] + water_duration[i]);
            result = result.min(water_start_time[i].max(t1) + water_duration[i]);
        }

        for i in 0..land_start_time.len() {
            result = result.min(land_start_time[i].max(t2) + land_duration[i]);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3635() {
        assert_eq!(
            Solution::earliest_finish_time(vec![2, 8], vec![4, 1], vec![6], vec![3]),
            9
        );
        assert_eq!(
            Solution::earliest_finish_time(vec![5], vec![3], vec![1], vec![10]),
            14
        );
    }
}
