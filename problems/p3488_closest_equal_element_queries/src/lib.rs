use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut nums_indices = vec![0; nums.len()];
        for i in 0..nums.len() {
            (*map.entry(nums[i]).or_insert(Vec::new())).push(i);
            nums_indices[i] = map.get(&nums[i]).unwrap().len() - 1;
        }

        queries
            .into_iter()
            .map(|q| {
                let i = q as usize;
                let indices = map.get(&nums[i]).unwrap();
                if indices.len() == 1 {
                    -1
                } else {
                    let mut d = std::usize::MAX;
                    if nums_indices[i] == 0 {
                        d = d.min(
                            indices[nums_indices[i]] + nums.len() - indices[indices.len() - 1],
                        );
                    } else {
                        d = d.min(indices[nums_indices[i]] - indices[nums_indices[i] - 1]);
                    }
                    if nums_indices[i] == indices.len() - 1 {
                        d = d.min(indices[0] + nums.len() - indices[indices.len() - 1]);
                    } else {
                        d = d.min(indices[nums_indices[i] + 1] - indices[nums_indices[i]]);
                    }
                    d as i32
                }
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3488() {
        assert_eq!(
            Solution::solve_queries(vec![1, 3, 1, 4, 1, 3, 2], vec![0, 3, 5]),
            vec![2, -1, 3]
        );
        assert_eq!(
            Solution::solve_queries(vec![1, 2, 3, 4], vec![0, 1, 2, 3]),
            vec![-1, -1, -1, -1]
        );
    }
}
