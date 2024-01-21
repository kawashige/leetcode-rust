pub struct Solution {}

impl Solution {
    pub fn smallest_trimmed_numbers(nums: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut trimed = vec![vec![]; nums[0].len() + 1];
        for i in 0..trimed.len() {
            for j in 0..nums.len() {
                trimed[i].push((nums[j][nums[j].len() - i..].to_string(), j));
            }
            trimed[i].sort_unstable();
        }

        queries
            .into_iter()
            .map(|q| trimed[q[1] as usize][q[0] as usize - 1].1 as i32)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2343() {
        assert_eq!(
            Solution::smallest_trimmed_numbers(
                vec![
                    "102".to_string(),
                    "473".to_string(),
                    "251".to_string(),
                    "814".to_string()
                ],
                vec![vec![1, 1], vec![2, 3], vec![4, 2], vec![1, 2]]
            ),
            vec![2, 2, 1, 0]
        );
        assert_eq!(
            Solution::smallest_trimmed_numbers(
                vec![
                    "24".to_string(),
                    "37".to_string(),
                    "96".to_string(),
                    "04".to_string()
                ],
                vec![vec![2, 1], vec![2, 2]]
            ),
            vec![3, 0]
        );
    }
}
