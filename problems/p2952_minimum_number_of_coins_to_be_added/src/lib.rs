pub struct Solution {}

impl Solution {
    pub fn minimum_added_coins(coins: Vec<i32>, target: i32) -> i32 {
        let mut coins = coins;
        coins.sort_unstable();
        let mut added = 0;
        let mut i = 0;
        let mut sum = 0;

        for x in 1..=target {
            while i < coins.len() && coins[i] <= x {
                sum += coins[i] as usize;
                i += 1;
            }
            if sum < x as usize {
                added += 1;
                sum += x as usize;
            }
        }

        added
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2952() {
        assert_eq!(Solution::minimum_added_coins(vec![1, 4, 10], 19), 2);
        assert_eq!(
            Solution::minimum_added_coins(vec![1, 4, 10, 5, 7, 19], 19),
            1
        );
        assert_eq!(Solution::minimum_added_coins(vec![1, 1, 1], 20), 3);
    }
}
