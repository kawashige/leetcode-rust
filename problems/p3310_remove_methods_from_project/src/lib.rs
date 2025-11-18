pub struct Solution {}

impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let mut list = vec![vec![]; n as usize];
        for i in 0..invocations.len() {
            list[invocations[i][0] as usize].push(invocations[i][1] as usize);
        }
        let mut is_suspicious = vec![false; n as usize];
        let mut stack = vec![k as usize];
        while let Some(node) = stack.pop() {
            if is_suspicious[node] {
                continue;
            }
            is_suspicious[node] = true;
            for next in &list[node] {
                stack.push(*next);
            }
        }
        if (0..invocations.len()).any(|i| {
            !is_suspicious[invocations[i][0] as usize] && is_suspicious[invocations[i][1] as usize]
        }) {
            (0..n).collect()
        } else {
            (0..n).filter(|i| !is_suspicious[*i as usize]).collect()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3310() {
        assert_eq!(
            Solution::remaining_methods(4, 1, vec![vec![1, 2], vec![0, 1], vec![3, 2]]),
            vec![0, 1, 2, 3]
        );
        assert_eq!(
            Solution::remaining_methods(5, 0, vec![vec![1, 2], vec![0, 2], vec![0, 1], vec![3, 4]]),
            vec![3, 4]
        );
        assert_eq!(
            Solution::remaining_methods(3, 2, vec![vec![1, 2], vec![0, 1], vec![2, 0]]),
            vec![]
        );
    }
}
