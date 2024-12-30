pub struct Solution {}

impl Solution {
    pub fn min_processing_time(processor_time: Vec<i32>, tasks: Vec<i32>) -> i32 {
        let mut processor_time = processor_time;
        let mut tasks = tasks;
        processor_time.sort_unstable_by_key(|a| -a);
        tasks.sort_unstable();
        (0..processor_time.len())
            .map(|i| {
                (0..4)
                    .map(|j| processor_time[i] + tasks[i * 4 + j])
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2895() {
        assert_eq!(
            Solution::min_processing_time(vec![8, 10], vec![2, 2, 3, 1, 8, 7, 4, 5]),
            16
        );
        assert_eq!(
            Solution::min_processing_time(vec![10, 20], vec![2, 3, 1, 2, 5, 8, 4, 3]),
            23
        );
    }
}
