pub struct Solution {}

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let mut sum = 0;
        let mut max_minutes_sum = 0;
        let mut minutes_sum = 0;

        for i in 0..customers.len() {
            if grumpy[i] == 0 {
                sum += customers[i];
            } else {
                minutes_sum += customers[i];
            }

            if minutes as usize <= i && grumpy[i - minutes as usize] == 1 {
                minutes_sum -= customers[i - minutes as usize];
            }

            max_minutes_sum = max_minutes_sum.max(minutes_sum);
        }

        sum + max_minutes_sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1052() {
        assert_eq!(
            Solution::max_satisfied(
                vec![1, 0, 1, 2, 1, 1, 7, 5],
                vec![0, 1, 0, 1, 0, 1, 0, 1],
                3
            ),
            16
        );
        assert_eq!(Solution::max_satisfied(vec![1], vec![0], 1), 1);
    }
}
