pub struct Solution {}

impl Solution {
    pub fn min_moves(balance: Vec<i32>) -> i64 {
        let mut i = balance.len();
        let mut sum = 0;
        for j in 0..balance.len() {
            sum += balance[j] as i64;
            if balance[j] < 0 {
                i = j;
            }
        }
        if i == balance.len() {
            return 0;
        }
        if sum < 0 {
            return -1;
        }
        let mut remains = balance[i] as i64;
        let mut d = 1;
        let mut result = 0;
        while remains < 0 {
            let value = ((balance[(i + d) % balance.len()]
                + balance[(i + balance.len() - d % balance.len()) % balance.len()])
                as i64)
                .min(remains.abs());
            result += value * d as i64;
            remains += value;
            d += 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3776() {
        assert_eq!(Solution::min_moves(vec![5, 1, -4]), 4);
        assert_eq!(Solution::min_moves(vec![1, 2, -5, 2]), 6);
        assert_eq!(Solution::min_moves(vec![-3, 2]), -1);
    }
}

fn main() {}
