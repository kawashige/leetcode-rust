pub struct Solution {}

impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        let mut power_of_threes = vec![1];
        while power_of_threes.last().unwrap() <= &10_000_000 {
            power_of_threes.push(power_of_threes.last().unwrap() * 3);
        }

        let mut remains = n;
        for i in (0..power_of_threes.len()).rev() {
            if power_of_threes[i] <= remains {
                remains -= power_of_threes[i];
            }
        }

        remains == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1780() {
        assert!(Solution::check_powers_of_three(12));
        assert!(Solution::check_powers_of_three(91));
        assert!(!Solution::check_powers_of_three(21));
    }
}
