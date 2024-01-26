pub struct Solution {}

impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut weights = vec![0; 1001];
        for item in items1 {
            weights[item[0] as usize] += item[1];
        }
        for item in items2 {
            weights[item[0] as usize] += item[1];
        }
        (0..weights.len())
            .filter_map(|i| {
                if weights[i] == 0 {
                    None
                } else {
                    Some(vec![i as i32, weights[i]])
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2363() {
        assert_eq!(
            Solution::merge_similar_items(
                vec![vec![1, 1], vec![4, 5], vec![3, 8]],
                vec![vec![3, 1], vec![1, 5]]
            ),
            vec![vec![1, 6], vec![3, 9], vec![4, 5]]
        );
        assert_eq!(
            Solution::merge_similar_items(
                vec![vec![1, 1], vec![3, 2], vec![2, 3]],
                vec![vec![2, 1], vec![3, 2], vec![1, 3]]
            ),
            vec![vec![1, 4], vec![2, 4], vec![3, 4]]
        );
        assert_eq!(
            Solution::merge_similar_items(
                vec![vec![1, 3], vec![2, 2]],
                vec![vec![7, 1], vec![2, 2], vec![1, 4]]
            ),
            vec![vec![1, 7], vec![2, 4], vec![7, 1]]
        );
    }
}
