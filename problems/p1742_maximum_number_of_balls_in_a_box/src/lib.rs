pub struct Solution {}

impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut count = vec![0; 50];

        for i in low_limit..=high_limit {
            count[i
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .sum::<usize>()] += 1;
        }

        count.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1742() {
        assert_eq!(Solution::count_balls(1, 100000), 6000);
        assert_eq!(Solution::count_balls(1, 10), 2);
        assert_eq!(Solution::count_balls(5, 15), 2);
        assert_eq!(Solution::count_balls(19, 28), 2);
    }
}
