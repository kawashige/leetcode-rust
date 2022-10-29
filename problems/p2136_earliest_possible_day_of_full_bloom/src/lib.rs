pub struct Solution {}

impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut plant = plant_time
            .into_iter()
            .zip(grow_time.into_iter())
            .collect::<Vec<_>>();
        plant.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        let mut blooming_day = 0;
        let mut current_day = 0;

        for (plant_time, grow_time) in plant {
            current_day += plant_time;
            blooming_day = blooming_day.max(current_day + grow_time);
        }

        blooming_day
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2136() {
        assert_eq!(
            Solution::earliest_full_bloom(vec![1, 4, 3], vec![2, 3, 1]),
            9
        );
        assert_eq!(
            Solution::earliest_full_bloom(vec![1, 2, 3, 2], vec![2, 1, 2, 1]),
            9
        );
        assert_eq!(Solution::earliest_full_bloom(vec![1], vec![1]), 2);
    }
}
