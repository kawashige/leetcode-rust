pub struct Solution {}

impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if ((m * n) as usize) != original.len() {
            return Default::default();
        }

        let mut result = vec![vec![0; n as usize]; m as usize];
        for i in 0..original.len() {
            result[i / n as usize][i % n as usize] = original[i];
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2022() {
        assert_eq!(
            Solution::construct2_d_array(vec![1, 2, 3, 4], 2, 2),
            vec![vec![1, 2], vec![3, 4]]
        );
        assert_eq!(
            Solution::construct2_d_array(vec![1, 2, 3], 1, 3),
            vec![vec![1, 2, 3]]
        );
        assert_eq!(
            Solution::construct2_d_array(vec![1, 2], 1, 1),
            vec![] as Vec<Vec<i32>>
        );
    }
}
