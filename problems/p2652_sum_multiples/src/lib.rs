pub struct Solution {}

impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        (1..=n)
            .filter(|i| i % 3 == 0 || i % 5 == 0 || i % 7 == 0)
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2652() {
        assert_eq!(Solution::sum_of_multiples(7), 21);
        assert_eq!(Solution::sum_of_multiples(10), 40);
        assert_eq!(Solution::sum_of_multiples(9), 30);
    }
}
