pub struct Solution {}

impl Solution {
    pub fn get_maximum_consecutive(mut coins: Vec<i32>) -> i32 {
        coins.sort_unstable();

        let mut x = 0;

        for i in 0..coins.len() {
            if x + 1 < coins[i] {
                break;
            }
            x += coins[i];
        }

        x + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1798() {
        assert_eq!(Solution::get_maximum_consecutive(vec![1, 3]), 2);
        assert_eq!(Solution::get_maximum_consecutive(vec![1, 1, 1, 4]), 8);
        assert_eq!(Solution::get_maximum_consecutive(vec![1, 4, 10, 3, 1]), 20);
    }
}
