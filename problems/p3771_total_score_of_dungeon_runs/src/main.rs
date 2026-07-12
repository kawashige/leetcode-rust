pub struct Solution {}

impl Solution {
    pub fn total_score(hp: i32, damage: Vec<i32>, requirement: Vec<i32>) -> i64 {
        let hp = hp as i64;
        let mut prefix_sum = Vec::with_capacity(damage.len());
        prefix_sum.push(0);
        let mut sum = 0;
        let mut result = 0;

        for i in 0..damage.len() {
            sum += damage[i] as i64;

            if hp < requirement[i] as i64 {
                prefix_sum.push(sum);
                continue;
            }

            result += prefix_sum.len()
                - prefix_sum
                    .binary_search(&(sum - hp + requirement[i] as i64))
                    .unwrap_or_else(|j| j);

            prefix_sum.push(sum);
        }

        result as i64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3771() {
        assert_eq!(
            Solution::total_score(
                6,
                vec![1, 10000, 1, 10000, 1, 10000],
                vec![1, 10000, 1, 10000, 1, 10000]
            ),
            3
        );
        assert_eq!(Solution::total_score(11, vec![3, 6, 7], vec![4, 2, 5]), 3);
        assert_eq!(Solution::total_score(2, vec![10000, 1], vec![1, 1]), 1);
    }
}

fn main() {}
