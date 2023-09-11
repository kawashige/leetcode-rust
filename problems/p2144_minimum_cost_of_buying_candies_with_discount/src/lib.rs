pub struct Solution {}

impl Solution {
    pub fn minimum_cost(cost: Vec<i32>) -> i32 {
        let mut cost = cost;
        cost.sort_unstable_by(|a, b| b.cmp(&a));

        (0..cost.len())
            .map(|i| if (i + 1) % 3 == 0 { 0 } else { cost[i] })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2144() {
        assert_eq!(Solution::minimum_cost(vec![1, 2, 3]), 5);
        assert_eq!(Solution::minimum_cost(vec![6, 5, 7, 9, 2, 2]), 23);
        assert_eq!(Solution::minimum_cost(vec![5, 5]), 10);
    }
}
