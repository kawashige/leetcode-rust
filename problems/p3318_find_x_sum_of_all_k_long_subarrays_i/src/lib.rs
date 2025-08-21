pub struct Solution {}

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let mut freq = vec![0; 51];
        for i in 0..k as usize - 1 {
            freq[nums[i] as usize] += 1;
        }
        for i in 0..nums.len() - k as usize + 1 {
            freq[nums[i + k as usize - 1] as usize] += 1;
            if 0 < i {
                freq[nums[i - 1] as usize] -= 1;
            }
            let mut sorted_freq = freq.iter().cloned().zip(0..).collect::<Vec<_>>();
            sorted_freq.sort_unstable();
            result.push(
                sorted_freq
                    .into_iter()
                    .rev()
                    .take(x as usize)
                    .map(|(c, n)| n * c)
                    .sum(),
            );
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3318() {
        assert_eq!(
            Solution::find_x_sum(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2),
            vec![6, 10, 12]
        );
        assert_eq!(
            Solution::find_x_sum(vec![3, 8, 7, 8, 7, 5], 2, 2),
            vec![11, 15, 15, 15, 12]
        );
    }
}
