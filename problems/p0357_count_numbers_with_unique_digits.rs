pub struct Solution {}

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        match n {
            0 => 1,
            1 => 10,
            _ => {
                let mut result = 10;
                let mut tmp = 9;
                for i in 1..n {
                    tmp *= 10 - i;
                    result += tmp;
                }
                result
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0357() {
        assert_eq!(1, Solution::count_numbers_with_unique_digits(0));
        assert_eq!(10, Solution::count_numbers_with_unique_digits(1));
        assert_eq!(91, Solution::count_numbers_with_unique_digits(2));
        assert_eq!(739, Solution::count_numbers_with_unique_digits(3));
    }
}
