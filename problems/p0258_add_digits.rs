pub struct Solution {}

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut n = num;
        while 10 <= n {
            n = n
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .sum::<i32>()
        }
        n
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0258() {
        assert_eq!(2, Solution::add_digits(38));
    }
}
