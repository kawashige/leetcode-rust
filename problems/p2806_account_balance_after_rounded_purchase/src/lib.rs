pub struct Solution {}

impl Solution {
    pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
        100 - (purchase_amount - purchase_amount % 10
            + if purchase_amount % 10 < 5 { 0 } else { 10 })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2806() {
        assert_eq!(Solution::account_balance_after_purchase(9), 90);
        assert_eq!(Solution::account_balance_after_purchase(15), 80);
        assert_eq!(Solution::account_balance_after_purchase(10), 90);
    }
}
