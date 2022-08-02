pub struct Solution {}

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut prev_color = b'A';
        let mut sum_cost = 0;
        let mut max_cost = 0;
        let mut total_cost = 0;

        for (i, b) in colors.as_bytes().iter().enumerate() {
            if &prev_color == b {
                sum_cost += needed_time[i];
                max_cost = max_cost.max(needed_time[i]);
            } else {
                total_cost += sum_cost - max_cost;
                max_cost = needed_time[i];
                sum_cost = needed_time[i];
                prev_color = *b;
            }
            println!(
                "{}: sum_cost: {}, min_cost: {}, total_cost: {}",
                i, sum_cost, max_cost, total_cost
            );
        }

        total_cost + sum_cost - max_cost
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1578() {
        assert_eq!(
            Solution::min_cost("abaac".to_string(), vec![1, 2, 3, 4, 5]),
            3
        );
        assert_eq!(Solution::min_cost("abc".to_string(), vec![1, 2, 3]), 0);
        assert_eq!(
            Solution::min_cost("aabaa".to_string(), vec![1, 2, 3, 4, 1]),
            2
        );
    }
}
