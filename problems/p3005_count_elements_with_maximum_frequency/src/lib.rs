pub struct Solution {}

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let count = nums.into_iter().fold([0; 101], |mut count, num| {
            count[num as usize] += 1;
            count
        });
        let max = count.iter().max().unwrap();
        count.into_iter().filter(|c| c == max).count() as i32 * max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3005() {
        assert_eq!(Solution::max_frequency_elements(vec![1, 2, 2, 3, 1, 4]), 4);
        assert_eq!(Solution::max_frequency_elements(vec![1, 2, 3, 4, 5]), 5);
    }
}
