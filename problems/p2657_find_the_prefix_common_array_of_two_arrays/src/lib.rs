pub struct Solution {}

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut count = vec![0; a.len() + 1];
        let mut common = 0;
        let mut result = vec![0; a.len()];

        for i in 0..a.len() {
            count[a[i] as usize] += 1;
            if count[a[i] as usize] == 2 {
                common += 1;
            }
            count[b[i] as usize] += 1;
            if count[b[i] as usize] == 2 {
                common += 1;
            }
            result[i] = common;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2657() {
        assert_eq!(
            Solution::find_the_prefix_common_array(vec![1, 3, 2, 4], vec![3, 1, 2, 4]),
            vec![0, 2, 3, 4]
        );
        assert_eq!(
            Solution::find_the_prefix_common_array(vec![2, 3, 1], vec![3, 1, 2]),
            vec![0, 1, 3]
        );
    }
}
