pub struct Solution {}

impl Solution {
    pub fn recurse(cur: usize, prev: usize, list: &Vec<Vec<usize>>, count: &mut i32) -> usize {
        let mut size = Vec::new();
        let mut same_size = true;
        for child in &list[cur] {
            if &prev == child {
                continue;
            }
            size.push(Self::recurse(*child, cur, list, count));
            if 1 < size.len() && size[size.len() - 1] != size[size.len() - 2] {
                same_size = false;
            }
        }
        if same_size {
            *count += 1;
        }
        size.into_iter().sum::<usize>() + 1
    }
    pub fn count_good_nodes(edges: Vec<Vec<i32>>) -> i32 {
        let mut list = vec![vec![]; edges.len() + 1];
        for i in 0..edges.len() {
            list[edges[i][0] as usize].push(edges[i][1] as usize);
            list[edges[i][1] as usize].push(edges[i][0] as usize);
        }
        let mut count = 0;
        Self::recurse(0, list.len(), &list, &mut count);
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3249() {
        assert_eq!(
            Solution::count_good_nodes(vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 5],
                vec![2, 6]
            ]),
            7
        );
        assert_eq!(
            Solution::count_good_nodes(vec![
                vec![0, 1],
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![0, 5],
                vec![1, 6],
                vec![2, 7],
                vec![3, 8]
            ]),
            6
        );
        assert_eq!(
            Solution::count_good_nodes(vec![
                vec![0, 1],
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![0, 5],
                vec![5, 6],
                vec![6, 7],
                vec![7, 8],
                vec![0, 9],
                vec![9, 10],
                vec![9, 12],
                vec![10, 11]
            ]),
            12
        );
    }
}
