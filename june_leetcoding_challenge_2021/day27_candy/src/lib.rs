pub struct Solution {}

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut count = vec![1; ratings.len()];

        for i in 1..ratings.len() {
            if ratings[i - 1] < ratings[i] {
                count[i] = count[i - 1] + 1;
            }
        }

        for i in (0..(ratings.len() - 1)).rev() {
            if ratings[i] > ratings[i + 1] {
                count[i] = std::cmp::max(count[i], count[i + 1] + 1);
            }
        }

        count.iter().sum::<i32>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day27() {
        assert_eq!(Solution::candy(vec![1, 2, 87, 87, 87, 2, 1]), 13);
        assert_eq!(Solution::candy(vec![0, 1, 0, 0]), 5);
        assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
        assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
    }
}
