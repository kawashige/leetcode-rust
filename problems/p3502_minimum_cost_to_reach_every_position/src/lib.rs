pub struct Solution {}

impl Solution {
    pub fn min_costs(cost: Vec<i32>) -> Vec<i32> {
        let mut result = vec![cost[0]; cost.len()];
        for i in 1..cost.len() {
            result[i] = result[i - 1].min(cost[i]);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3502() {
        assert_eq!(
            Solution::min_costs(vec![5, 3, 4, 1, 3, 2]),
            vec![5, 3, 3, 1, 1, 1]
        );
        assert_eq!(
            Solution::min_costs(vec![1, 2, 4, 6, 7]),
            vec![1, 1, 1, 1, 1]
        );
    }
}
