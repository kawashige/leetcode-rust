pub struct Solution {}

impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter()
            .fold([0; 101], |mut count, num| {
                count[num as usize] += 1;
                count
            })
            .into_iter()
            .fold(vec![0; 2], |acc, count| {
                vec![acc[0] + count / 2, acc[1] + count % 2]
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2341() {
        assert_eq!(
            Solution::number_of_pairs(vec![1, 3, 2, 1, 3, 2, 2]),
            vec![3, 1]
        );
        assert_eq!(Solution::number_of_pairs(vec![1, 1]), vec![1, 0]);
        assert_eq!(Solution::number_of_pairs(vec![0]), vec![0, 1]);
    }
}
