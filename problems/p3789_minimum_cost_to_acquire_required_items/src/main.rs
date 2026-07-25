pub struct Solution {}

impl Solution {
    pub fn minimum_cost(cost1: i32, cost2: i32, cost_both: i32, need1: i32, need2: i32) -> i64 {
        if need1 == 0 && need2 == 0 {
            0
        } else if need1 == 0 {
            need2 as i64 * cost2.min(cost_both) as i64
        } else if need2 == 0 {
            need1 as i64 * cost1.min(cost_both) as i64
        } else {
            let mut result = 0;
            result += need1.min(need2) as i64 * (cost1 + cost2).min(cost_both) as i64;
            if need1 < need2 {
                result += (need2 - need1) as i64 * cost2.min(cost_both) as i64;
            } else {
                result += (need1 - need2) as i64 * cost1.min(cost_both) as i64;
            }
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3789() {
        assert_eq!(Solution::minimum_cost(3, 2, 1, 3, 2), 3);
        assert_eq!(Solution::minimum_cost(5, 4, 15, 2, 3), 22);
        assert_eq!(Solution::minimum_cost(5, 4, 15, 0, 0), 0);
    }
}

fn main() {}
