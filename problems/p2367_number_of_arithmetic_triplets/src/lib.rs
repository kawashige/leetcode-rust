pub struct Solution {}

impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let diff = diff as usize;
        let mut count = vec![0; 201];
        for num in nums {
            count[num as usize] += 1;
        }
        (0..count.len() - diff * 2)
            .map(|i| count[i] * count[i + diff] * count[i + diff * 2])
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2367() {
        assert_eq!(Solution::arithmetic_triplets(vec![0, 1, 4, 6, 7, 10], 3), 2);
        assert_eq!(Solution::arithmetic_triplets(vec![4, 5, 6, 7, 8, 9], 2), 2);
    }
}
