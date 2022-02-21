pub struct Solution {}

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let (mul, sum) = n.to_string().chars().fold((1, 0), |(m, s), c| {
            let d = c.to_digit(10).unwrap() as i32;
            (m * d, s + d)
        });

        mul - sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1281() {
        assert_eq!(Solution::subtract_product_and_sum(234), 15);
        assert_eq!(Solution::subtract_product_and_sum(4421), 21);
    }
}
