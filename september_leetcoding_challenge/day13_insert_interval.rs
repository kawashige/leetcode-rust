pub struct Solution {}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        fn not_overlapped(i1: &Vec<i32>, i2: &Vec<i32>) -> bool {
            i1[1] < i2[0] || i2[1] < i1[0]
        }
        let mut results = Vec::new();
        let mut tmp = new_interval;
        let mut inserted = false;
        for v in intervals {
            if inserted {
                results.push(v);
            } else {
                if not_overlapped(&tmp, &v) {
                    if tmp[1] < v[0] {
                        results.push(tmp.clone());
                        inserted = true;
                    }
                    results.push(v);
                } else {
                    tmp = vec![std::cmp::min(tmp[0], v[0]), std::cmp::max(tmp[1], v[1])];
                }
            }
        }
        if !inserted {
            results.push(tmp);
        }
        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day13() {
        assert_eq!(
            vec![vec![2, 3], vec![4, 5], vec![6, 9]],
            Solution::insert(vec![vec![4, 5], vec![6, 9]], vec![2, 3])
        );
        assert_eq!(
            vec![vec![4, 5], vec![6, 9], vec![10, 13]],
            Solution::insert(vec![vec![4, 5], vec![6, 9]], vec![10, 13])
        );
        assert_eq!(
            vec![vec![1, 5], vec![6, 9]],
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5])
        );
        assert_eq!(
            vec![vec![1, 2], vec![3, 10], vec![12, 16]],
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            )
        );
    }
}
