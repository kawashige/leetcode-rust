pub struct Solution {}

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut size_with_index = group_sizes.into_iter().enumerate().collect::<Vec<_>>();
        size_with_index.sort_unstable_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

        let mut result = Vec::new();
        let mut tmp = Vec::new();
        let mut size = 0;

        for (i, s) in size_with_index {
            if size == 0 {
                size = s;
            }
            tmp.push(i as i32);
            if tmp.len() as i32 == size {
                result.push(tmp);
                tmp = Vec::new();
                size = 0;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1282() {
        assert_eq!(
            Solution::group_the_people(vec![3, 3, 3, 3, 3, 1, 3]),
            vec![vec![5], vec![0, 1, 2], vec![3, 4, 6]]
        );
        assert_eq!(
            Solution::group_the_people(vec![2, 1, 3, 3, 3, 2]),
            vec![vec![1], vec![0, 5], vec![2, 3, 4]]
        );
    }
}
