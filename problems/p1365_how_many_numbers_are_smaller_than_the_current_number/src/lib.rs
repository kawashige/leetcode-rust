pub struct Solution {}

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut count = nums.iter().fold([0; 102], |mut count, x| {
            count[*x as usize + 1] += 1;
            count
        });
        for i in 1..count.len() {
            count[i] += count[i - 1];
        }
        nums.into_iter().map(|x| count[x as usize]).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1365() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        );
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![6, 5, 4, 8]),
            vec![2, 1, 0, 3]
        );
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![7, 7, 7, 7]),
            vec![0, 0, 0, 0]
        );
    }
}
