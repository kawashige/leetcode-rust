pub struct Solution {}

impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut days = 0;
        let mut smooth_decent_peridos = 0;

        for i in 0..prices.len() {
            if 0 < i && prices[i - 1] == prices[i] + 1 {
                smooth_decent_peridos += 1;
            } else {
                smooth_decent_peridos = 1;
            }
            days += smooth_decent_peridos;
        }

        days
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2110() {
        assert_eq!(Solution::get_descent_periods(vec![3, 2, 1, 4]), 7);
        assert_eq!(Solution::get_descent_periods(vec![8, 6, 7, 7]), 4);
        assert_eq!(Solution::get_descent_periods(vec![1]), 1);
    }
}
