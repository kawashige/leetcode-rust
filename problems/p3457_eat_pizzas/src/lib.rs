pub struct Solution {}

impl Solution {
    pub fn max_weight(pizzas: Vec<i32>) -> i64 {
        let mut pizzas = pizzas;
        pizzas.sort_unstable_by_key(|p| -p);

        let n = pizzas.len() / 4;
        let mut c = 0;
        let mut result = 0;

        for i in 0..(n + 1) / 2 {
            result += pizzas[i] as i64;
            c += 1;
        }
        for i in ((n + 1) / 2 + 1..pizzas.len()).step_by(2) {
            if c == n {
                break;
            }
            result += pizzas[i] as i64;
            c += 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3457() {
        assert_eq!(Solution::max_weight(vec![1, 2, 3, 4, 5, 6, 7, 8]), 14);
        assert_eq!(Solution::max_weight(vec![2, 1, 1, 1, 1, 1, 1, 1]), 3);
    }
}
