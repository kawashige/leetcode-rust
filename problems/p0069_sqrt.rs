pub struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        (x as f64).sqrt().floor() as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_69() {
        assert_eq!(2, Solution::my_sqrt(4));
    }
}
