pub struct Solution {}

impl Solution {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut items = items;
        items.sort_unstable();
        let mut queries = queries.into_iter().zip(0..).collect::<Vec<_>>();
        queries.sort_unstable();

        let mut result = vec![0; queries.len()];
        let mut max_beauty = 0;
        let mut i = 0;

        for q in queries {
            while i < items.len() && items[i][0] <= q.0 {
                max_beauty = max_beauty.max(items[i][1]);
                i += 1;
            }
            result[q.1] = max_beauty;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2070() {
        assert_eq!(
            Solution::maximum_beauty(
                vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]],
                vec![1, 2, 3, 4, 5, 6]
            ),
            vec![2, 4, 5, 5, 6, 6]
        );
        assert_eq!(
            Solution::maximum_beauty(
                vec![vec![1, 2], vec![1, 2], vec![1, 3], vec![1, 4]],
                vec![1]
            ),
            vec![4]
        );
        assert_eq!(
            Solution::maximum_beauty(vec![vec![10, 1000]], vec![5]),
            vec![0]
        );
    }
}
