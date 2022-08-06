pub struct Solution {}

impl Solution {
    pub fn max_sum_range_query(mut nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
        const M: usize = 1_000_000_007;

        let mut count = vec![0; nums.len() + 1];
        for req in &requests {
            count[req[0] as usize] += 1;
            count[req[1] as usize + 1] -= 1;
        }
        for i in 1..count.len() {
            count[i] += count[i - 1];
        }

        count.sort_unstable_by(|a, b| b.cmp(&a));
        nums.sort_unstable_by(|a, b| b.cmp(&a));

        let mut sum = 0;
        for (c, num) in count.into_iter().zip(nums.into_iter()) {
            if c == 0 {
                break;
            }
            sum = (sum + (c as usize * num as usize) % M) % M
        }

        sum as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1589() {
        assert_eq!(
            Solution::max_sum_range_query(vec![1, 2, 3, 4, 5], vec![vec![1, 3], vec![0, 1]]),
            19
        );
        assert_eq!(
            Solution::max_sum_range_query(vec![1, 2, 3, 4, 5, 6], vec![vec![0, 1]]),
            11
        );
        assert_eq!(
            Solution::max_sum_range_query(
                vec![1, 2, 3, 4, 5, 10],
                vec![vec![0, 2], vec![1, 3], vec![1, 1]]
            ),
            47
        );
    }
}
