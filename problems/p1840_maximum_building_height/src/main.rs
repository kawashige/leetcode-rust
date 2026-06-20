pub struct Solution {}

impl Solution {
    pub fn max_building(n: i32, restrictions: Vec<Vec<i32>>) -> i32 {
        let mut restrictions = restrictions;
        restrictions.push(vec![1, 0]);
        restrictions.sort_unstable_by_key(|r| r[0]);
        if !(!restrictions.is_empty() && restrictions.last().unwrap()[0] == n) {
            restrictions.push(vec![n, n - 1]);
        }

        for i in 1..restrictions.len() {
            restrictions[i][1] = restrictions[i][1]
                .min(restrictions[i - 1][1] + restrictions[i][0] - restrictions[i - 1][0]);
        }

        for i in (0..restrictions.len() - 1).rev() {
            restrictions[i][1] = restrictions[i][1]
                .min(restrictions[i + 1][1] + restrictions[i + 1][0] - restrictions[i][0]);
        }

        let mut result = 0;
        for i in 0..restrictions.len() - 1 {
            result = result.max(
                (restrictions[i + 1][0] - restrictions[i][0]
                    + restrictions[i][1]
                    + restrictions[i + 1][1])
                    / 2,
            );
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1840() {
        assert_eq!(
            Solution::max_building(
                10,
                vec![
                    vec![8, 5],
                    vec![9, 0],
                    vec![6, 2],
                    vec![4, 0],
                    vec![3, 2],
                    vec![10, 0],
                    vec![5, 3],
                    vec![7, 3],
                    vec![2, 4]
                ]
            ),
            2
        );
        assert_eq!(Solution::max_building(5, vec![vec![2, 1], vec![4, 1]]), 2);
        assert_eq!(Solution::max_building(6, vec![]), 5);
        assert_eq!(
            Solution::max_building(10, vec![vec![5, 3], vec![2, 5], vec![7, 4], vec![10, 3]]),
            5
        );
    }
}

fn main() {}
