pub struct Solution {}

impl Solution {
    pub fn calc_distance(i: usize, d: i32, list: &Vec<Vec<usize>>, dist: &mut Vec<i32>) {
        dist[i] = d;
        for k in &list[i] {
            if dist[*k] == -1 {
                Self::calc_distance(*k, d + 1, list, dist);
            }
        }
    }
    pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
        let mut list1 = vec![vec![]; edges1.len() + 1];
        for edge in edges1 {
            list1[edge[0] as usize].push(edge[1] as usize);
            list1[edge[1] as usize].push(edge[0] as usize);
        }
        let mut dist1 = vec![-1; list1.len()];
        Self::calc_distance(0, 0, &list1, &mut dist1);
        let max_i = (0..dist1.len()).max_by_key(|i| dist1[*i]).unwrap();
        let mut dist1 = vec![-1; list1.len()];
        Self::calc_distance(max_i, 0, &list1, &mut dist1);
        let d1 = dist1.into_iter().max().unwrap();

        let mut list2 = vec![vec![]; edges2.len() + 1];
        for edge in edges2 {
            list2[edge[0] as usize].push(edge[1] as usize);
            list2[edge[1] as usize].push(edge[0] as usize);
        }
        let mut dist2 = vec![-1; list2.len()];
        Self::calc_distance(0, 0, &list2, &mut dist2);
        let max_i = (0..dist2.len()).max_by_key(|i| dist2[*i]).unwrap();
        let mut dist2 = vec![-1; list2.len()];
        Self::calc_distance(max_i, 0, &list2, &mut dist2);
        let d2 = dist2.into_iter().max().unwrap();

        (d1 - d1 / 2 + d2 - d2 / 2 + 1).max(d1).max(d2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3203() {
        assert_eq!(
            Solution::minimum_diameter_after_merge(
                vec![vec![0, 1], vec![0, 2], vec![0, 3]],
                vec![vec![0, 1]]
            ),
            3
        );
        assert_eq!(
            Solution::minimum_diameter_after_merge(
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 3],
                    vec![2, 4],
                    vec![2, 5],
                    vec![3, 6],
                    vec![2, 7]
                ],
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 3],
                    vec![2, 4],
                    vec![2, 5],
                    vec![3, 6],
                    vec![2, 7]
                ]
            ),
            5
        );
    }
}
