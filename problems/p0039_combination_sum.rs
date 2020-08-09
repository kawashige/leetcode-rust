pub struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn find(candidates: &[i32], selected: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
            let sum = selected.iter().sum::<i32>();
            if sum == target {
                return vec![selected.to_vec()];
            }
            if sum > target || candidates.len() == 0 {
                return vec![];
            }
            let mut new_selected = selected.clone();
            new_selected.push(candidates[0]);
            let mut result1 = find(&candidates, new_selected, target);
            let mut result2 = find(&candidates[1..], selected, target);
            result1.append(&mut result2);
            result1
        }
        find(&candidates, vec![], target)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0039() {
        assert_eq!(
            vec![vec![2, 2, 3], vec![7]],
            Solution::combination_sum(vec![2, 3, 6, 7], 7)
        );
        assert_eq!(
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]],
            Solution::combination_sum(vec![2, 3, 5], 8)
        );
        assert_eq!(vec![vec![2, 2]], Solution::combination_sum(vec![2], 4));
    }
}
