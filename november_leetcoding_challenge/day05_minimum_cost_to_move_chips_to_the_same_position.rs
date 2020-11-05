pub struct Solution {}

impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let (even_count, odd_count) = position.into_iter().fold((0, 0), |acc, n| match n % 2 {
            0 => (acc.0 + 1, acc.1),
            _ => (acc.0, acc.1 + 1),
        });

        if odd_count == 0 || even_count == 0 {
            0
        } else {
            std::cmp::min(odd_count, even_count)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day5() {
        assert_eq!(2, Solution::min_cost_to_move_chips(vec![2, 2, 2, 3, 3]));
        assert_eq!(1, Solution::min_cost_to_move_chips(vec![1, 1000000000]));
    }
}
