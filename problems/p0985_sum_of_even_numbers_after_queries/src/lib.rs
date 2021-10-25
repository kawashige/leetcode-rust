pub struct Solution {}

impl Solution {
    pub fn sum_even_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut even = nums.iter().filter(|x| **x % 2 == 0).sum::<i32>();
        queries
            .into_iter()
            .map(|v| {
                if nums[v[1] as usize] % 2 == 0 {
                    even -= nums[v[1] as usize];
                }
                nums[v[1] as usize] += v[0];
                if nums[v[1] as usize] % 2 == 0 {
                    even += nums[v[1] as usize];
                }
                even
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0985() {
        assert_eq!(
            Solution::sum_even_after_queries(
                vec![1, 2, 3, 4],
                vec![vec![1, 0], vec![-3, 1], vec![-4, 0], vec![2, 3]]
            ),
            vec![8, 6, 2, 4]
        );
        assert_eq!(
            Solution::sum_even_after_queries(vec![1], vec![vec![4, 0]]),
            vec![0]
        );
    }
}
