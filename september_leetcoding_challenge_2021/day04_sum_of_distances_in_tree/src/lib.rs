pub struct Solution {}

impl Solution {
    pub fn recurse(
        i: usize,
        parent: usize,
        list: &Vec<Vec<usize>>,
        child_count: &mut Vec<i32>,
        v: &mut Vec<i32>,
    ) {
        for next in &list[i] {
            if next != &parent {
                Self::recurse(*next, i, list, child_count, v);
                child_count[i] += child_count[*next];
                v[i] += child_count[*next] + v[*next];
            }
        }
    }

    pub fn calc(
        i: usize,
        parent: usize,
        list: &Vec<Vec<usize>>,
        child_count: &mut Vec<i32>,
        r: &mut Vec<i32>,
    ) {
        for next in &list[i] {
            if next != &parent {
                r[*next] = r[i] - child_count[*next] + list.len() as i32 - child_count[*next];
                Self::calc(*next, i, list, child_count, r);
            }
        }
    }

    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut list = vec![vec![]; n as usize];
        for edge in edges {
            list[edge[0] as usize].push(edge[1] as usize);
            list[edge[1] as usize].push(edge[0] as usize);
        }

        let mut child_count = vec![1; n as usize];
        let mut v = vec![0; n as usize];

        Self::recurse(0, n as usize, &list, &mut child_count, &mut v);

        let mut r = vec![0; n as usize];
        r[0] = v[0];

        Self::calc(0, n as usize, &list, &mut child_count, &mut r);

        r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day04() {
        assert_eq!(
            Solution::sum_of_distances_in_tree(
                6,
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]]
            ),
            vec![8, 12, 6, 10, 10, 10]
        );
        assert_eq!(Solution::sum_of_distances_in_tree(1, vec![]), vec![0]);
        assert_eq!(
            Solution::sum_of_distances_in_tree(2, vec![vec![1, 0]]),
            vec![1, 1]
        );
    }
}
