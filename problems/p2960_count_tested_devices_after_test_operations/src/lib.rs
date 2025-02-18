pub struct Solution {}

impl Solution {
    pub fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..battery_percentages.len() {
            if result < battery_percentages[i] {
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2960() {
        assert_eq!(Solution::count_tested_devices(vec![1, 1, 2, 1, 3]), 3);
        assert_eq!(Solution::count_tested_devices(vec![0, 1, 2]), 2);
    }
}
