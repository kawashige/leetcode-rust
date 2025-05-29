pub struct Solution {}

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        let mut list1 = vec![vec![]; edges1.len() + 1];
        for i in 0..edges1.len() {
            list1[edges1[i][0] as usize].push(edges1[i][1] as usize);
            list1[edges1[i][1] as usize].push(edges1[i][0] as usize);
        }
        let mut list2 = vec![vec![]; edges2.len() + 1];
        for i in 0..edges2.len() {
            list2[edges2[i][0] as usize].push(edges2[i][1] as usize);
            list2[edges2[i][1] as usize].push(edges2[i][0] as usize);
        }

        let mut group1 = vec![2; list1.len()];
        let mut count1 = vec![0; 2];
        let mut stack = vec![(0, 0)];
        while let Some((n, g)) = stack.pop() {
            if group1[n] != 2 {
                continue;
            }
            count1[g] += 1;
            group1[n] = g;
            for next in &list1[n] {
                stack.push((*next, (g + 1) % 2));
            }
        }

        let mut group2 = vec![2; list2.len()];
        let mut count2 = vec![0; 2];
        let mut stack = vec![(0, 0)];
        while let Some((n, g)) = stack.pop() {
            if group2[n] != 2 {
                continue;
            }
            count2[g] += 1;
            group2[n] = g;
            for next in &list2[n] {
                stack.push((*next, (g + 1) % 2));
            }
        }

        let mut result = vec![0; list1.len()];
        let max = count2[0].max(count2[1]);
        for i in 0..list1.len() {
            result[i] = count1[group1[i]] + max;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3373() {
        assert_eq!(
            Solution::max_target_nodes(
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]],
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 3],
                    vec![2, 7],
                    vec![1, 4],
                    vec![4, 5],
                    vec![4, 6]
                ]
            ),
            vec![8, 7, 7, 8, 8]
        );
        assert_eq!(
            Solution::max_target_nodes(
                vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]],
                vec![vec![0, 1], vec![1, 2], vec![2, 3]]
            ),
            vec![3, 6, 6, 6, 6]
        );
    }
}
