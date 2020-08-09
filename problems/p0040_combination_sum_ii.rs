pub struct Solution {}

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
            let mut result1 = find(&candidates[1..], new_selected, target);
            let mut i = 1;
            while i < candidates.len() && candidates[0] == candidates[i] {
                i += 1;
            }
            let mut result2 = find(&candidates[i..], selected, target);
            result1.append(&mut result2);
            result1
        }
        let mut c = candidates.clone();
        c.sort();
        find(&c, vec![], target)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0040() {
        assert_eq!(
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6],],
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8)
        );
        assert_eq!(
            vec![vec![1, 2, 2], vec![5]],
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5)
        );
    }
}
