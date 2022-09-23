pub struct Solution {}

impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut time = 0;
        let mut wait_time = 0;
        for c in &customers {
            time = time.max(c[0] as usize) + c[1] as usize;
            wait_time += time - c[0] as usize;
        }

        wait_time as f64 / customers.len() as f64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1701() {
        assert_eq!(
            Solution::average_waiting_time(vec![vec![1, 2], vec![2, 5], vec![4, 3]]),
            5.0
        );
        assert_eq!(
            Solution::average_waiting_time(vec![vec![5, 2], vec![5, 4], vec![10, 3], vec![20, 1]]),
            3.25
        );
    }
}
