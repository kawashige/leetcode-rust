pub struct Solution {}

impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let range = start.min(destination)..start.max(destination);
        let mut sum = 0;
        let mut range_sum = 0;

        for i in 0..distance.len() {
            sum += distance[i];
            if range.contains(&(i as i32)) {
                range_sum += distance[i];
            }
        }

        range_sum.min(sum - range_sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1184() {
        assert_eq!(
            Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 0),
            0
        );
        assert_eq!(
            Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 1),
            1
        );
        assert_eq!(
            Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 2),
            3
        );
        assert_eq!(
            Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 3),
            4
        );
    }
}
