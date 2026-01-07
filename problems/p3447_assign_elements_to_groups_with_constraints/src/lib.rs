pub struct Solution {}

impl Solution {
    pub fn assign_elements(groups: Vec<i32>, elements: Vec<i32>) -> Vec<i32> {
        let mut index = vec![-1; 100_001];
        for i in 0..elements.len() {
            if index[elements[i] as usize] != -1 {
                continue;
            }
            for j in (elements[i] as usize..index.len()).step_by(elements[i] as usize) {
                if index[j] != -1 {
                    continue;
                }
                index[j] = i as i32;
            }
        }
        groups.into_iter().map(|g| index[g as usize]).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3447() {
        assert_eq!(
            Solution::assign_elements(vec![8, 4, 3, 2, 4], vec![4, 2]),
            vec![0, 0, -1, 1, 0]
        );
        assert_eq!(
            Solution::assign_elements(vec![2, 3, 5, 7], vec![5, 3, 3]),
            vec![-1, 1, 0, -1]
        );
        assert_eq!(
            Solution::assign_elements(vec![10, 21, 30, 41], vec![2, 1]),
            vec![0, 1, 0, 1]
        );
    }
}
