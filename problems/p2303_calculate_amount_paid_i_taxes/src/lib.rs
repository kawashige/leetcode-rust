pub struct Solution {}

impl Solution {
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
        let mut remains = income;
        let mut taxes = 0;

        for i in 0..brackets.len() {
            if remains == 0 {
                break;
            }
            let amount =
                (brackets[i][0] - if i == 0 { 0 } else { brackets[i - 1][0] }).min(remains);
            println!("{}: {}", i, amount);
            remains -= amount;
            taxes += amount * brackets[i][1];
        }

        taxes as f64 / 100.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2303() {
        assert_eq!(
            Solution::calculate_tax(vec![vec![3, 50], vec![7, 10], vec![12, 25]], 10),
            2.65000
        );
        assert_eq!(
            Solution::calculate_tax(vec![vec![1, 0], vec![4, 25], vec![5, 50]], 2),
            0.25000
        );
        assert_eq!(Solution::calculate_tax(vec![vec![2, 50]], 0), 0.00000);
    }
}
