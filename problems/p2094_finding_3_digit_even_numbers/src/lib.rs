use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut set = HashSet::new();

        for i in 0..digits.len() {
            if digits[i] % 2 == 1 {
                continue;
            }
            for j in 0..digits.len() {
                if i == j {
                    continue;
                }
                for k in 0..digits.len() {
                    if i == k || j == k || digits[k] == 0 {
                        continue;
                    }
                    set.insert((digits[i] + 10 * digits[j] + 100 * digits[k]) as i32);
                }
            }
        }

        let mut result = set.into_iter().collect::<Vec<_>>();
        result.sort_unstable();
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2094() {
        assert_eq!(
            Solution::find_even_numbers(vec![2, 1, 3, 0]),
            vec![102, 120, 130, 132, 210, 230, 302, 310, 312, 320]
        );
        assert_eq!(
            Solution::find_even_numbers(vec![2, 2, 8, 8, 2]),
            vec![222, 228, 282, 288, 822, 828, 882]
        );
        assert_eq!(Solution::find_even_numbers(vec![3, 7, 5]), vec![]);
    }
}
