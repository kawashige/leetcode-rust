use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut count = arr
            .into_iter()
            .fold(HashMap::new(), |mut count, x| {
                *count.entry(x).or_insert(0) += 1;
                count
            })
            .values()
            .cloned()
            .collect::<Vec<_>>();
        count.sort_unstable();

        let mut remains = k;
        let mut result = count.len() as i32;
        for x in count {
            remains -= x;
            if remains < 0 {
                break;
            }
            result -= 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1481() {
        assert_eq!(Solution::find_least_num_of_unique_ints(vec![5, 5, 4], 1), 1);
        assert_eq!(
            Solution::find_least_num_of_unique_ints(vec![4, 3, 1, 1, 3, 3, 2], 3),
            2
        );
    }
}
