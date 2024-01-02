pub struct Solution {}

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut count = vec![0; nums.len() + 1];
        for num in nums {
            count[num as usize] += 1;
        }
        let mut result = vec![];
        for i in 1..count.len() {
            while result.len() < count[i] {
                result.push(Vec::new());
            }
            for j in 0..count[i] {
                result[j].push(i as i32);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2610() {
        assert_eq!(
            Solution::find_matrix(vec![1, 3, 4, 1, 2, 3, 1]),
            vec![vec![1, 2, 3, 4], vec![1, 3], vec![1]]
        );
        assert_eq!(
            Solution::find_matrix(vec![1, 2, 3, 4]),
            vec![vec![1, 2, 3, 4]]
        );
    }
}
