pub struct Solution {}

impl Solution {
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        let mut single = 0;
        let mut double = 0;
        let mut total = 0;
        for n in nums {
            total += n;
            if n < 10 {
                single += n;
            } else if (10..100).contains(&n) {
                double += n;
            }
        }

        total - single < single || total - double < double
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3232() {
        assert!(!Solution::can_alice_win(vec![1, 2, 3, 4, 10]));
        assert!(Solution::can_alice_win(vec![1, 2, 3, 4, 5, 14]));
        assert!(Solution::can_alice_win(vec![5, 5, 5, 25]));
    }
}
