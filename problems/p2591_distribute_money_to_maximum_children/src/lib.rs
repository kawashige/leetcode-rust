pub struct Solution {}

impl Solution {
    pub fn dist_money(money: i32, children: i32) -> i32 {
        if money < children || (money == 4 && children == 1) {
            return -1;
        }
        let money = money - children;

        for i in (1..=(money / 7).min(children)).rev() {
            if (children - i == 0 && 0 < money - 7 * i) || (children - i == 1 && money - 7 * i == 3)
            {
                continue;
            }
            return i;
        }

        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2591() {
        assert_eq!(Solution::dist_money(20, 3), 1);
        assert_eq!(Solution::dist_money(16, 2), 2);
    }
}
