pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut matrix = HashMap::new();

        for p in prerequisites {
            matrix.entry(p[0]).or_insert(Vec::new()).push(p[1]);
        }

        let mut result = Vec::new();
        for n in 0..num_courses {
            if !matrix.contains_key(&n) {
                result.push(n);
            }
        }

        while matrix.len() > 0 {
            {
                let num = matrix
                    .keys()
                    .find(|k| matrix[k].iter().all(|x| !matrix.contains_key(&x)));

                if num.is_none() {
                    return Vec::new();
                }

                result.push(*num.unwrap());
            }
            matrix.remove(result.last().unwrap());
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0210() {
        assert_eq!(vec![0, 1], Solution::find_order(2, vec![vec![1, 0]]));
        assert_eq!(
            vec![0, 2, 1, 3],
            Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]])
        );
        assert_eq!(vec![0], Solution::find_order(1, vec![]));
        assert_eq!(
            vec![] as Vec<i32>,
            Solution::find_order(2, vec![vec![0, 1], vec![1, 0]])
        );
    }
}
