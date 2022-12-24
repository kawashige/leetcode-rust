pub struct Solution {}

impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, coins: i32) -> i32 {
        costs.sort_unstable();
        let mut sum = 0;
        for i in 0..costs.len() {
            if coins < sum + costs[i] {
                return i as i32;
            }
            sum += costs[i];
        }
        costs.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1833() {
        assert_eq!(Solution::max_ice_cream(vec![1, 3, 2, 4, 1], 7), 4);
        assert_eq!(Solution::max_ice_cream(vec![10, 6, 8, 7, 8, 8], 5), 0);
        assert_eq!(Solution::max_ice_cream(vec![1, 6, 3, 1, 2, 5], 20), 6);
    }
}
