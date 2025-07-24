pub struct Solution {}

impl Solution {
    pub fn recurse(
        node: usize,
        prev_node: usize,
        cur_xor: i32,
        nums: &[i32],
        list: &Vec<Vec<usize>>,
        all_xor: i32,
        xor_value: i32,
        result: &mut i32,
    ) -> i32 {
        let mut subtree_xors = Vec::new();
        let mut subtree_all_xor = 0;
        for next in &list[node] {
            if next == &prev_node {
                continue;
            }
            let subtree_xor = Self::recurse(
                *next,
                node,
                cur_xor ^ nums[*next],
                nums,
                list,
                all_xor,
                xor_value,
                result,
            );
            subtree_all_xor ^= subtree_xor;
            subtree_xors.push(subtree_xor);
        }
        for i in 0..subtree_xors.len() {
            let v1 = all_xor ^ xor_value;
            let v2 = subtree_xors[i];
            let v3 = xor_value ^ subtree_xors[i];
            let min = v1.min(v2).min(v3);
            let max = v1.max(v2).max(v3);
            if &(max - min) < result {
                *result = max - min;
            }
        }

        subtree_all_xor ^ nums[node]
    }

    pub fn calc_subtree_xor(cur: usize, prev: usize, list: &Vec<Vec<usize>>, nums: &[i32]) -> i32 {
        let mut xor_value = 0;
        for next in &list[cur] {
            if next == &prev {
                continue;
            }
            xor_value ^= Self::calc_subtree_xor(*next, cur, list, nums);
        }
        xor_value ^ nums[cur]
    }

    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let mut list = vec![vec![]; nums.len()];
        for i in 0..edges.len() {
            list[edges[i][0] as usize].push(edges[i][1] as usize);
            list[edges[i][1] as usize].push(edges[i][0] as usize);
        }
        let all_xor = nums.iter().fold(0, |acc, x| acc ^ x);

        let mut result = std::i32::MAX;
        for i in 0..edges.len() {
            let xor_value =
                Self::calc_subtree_xor(edges[i][0] as usize, edges[i][1] as usize, &list, &nums);
            Self::recurse(
                edges[i][0] as usize,
                edges[i][1] as usize,
                nums[edges[i][0] as usize],
                &nums,
                &list,
                all_xor,
                xor_value,
                &mut result,
            );
            let xor_value =
                Self::calc_subtree_xor(edges[i][1] as usize, edges[i][0] as usize, &list, &nums);
            Self::recurse(
                edges[i][1] as usize,
                edges[i][0] as usize,
                nums[edges[i][1] as usize],
                &nums,
                &list,
                all_xor,
                xor_value,
                &mut result,
            );
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2322() {
        assert_eq!(
            Solution::minimum_score(
                vec![10, 15, 24, 18, 30, 14, 31, 14, 8, 16, 23, 16, 4, 9, 26, 17],
                vec![
                    vec![2, 8],
                    vec![5, 8],
                    vec![3, 5],
                    vec![6, 8],
                    vec![0, 8],
                    vec![9, 0],
                    vec![7, 8],
                    vec![10, 5],
                    vec![10, 12],
                    vec![4, 3],
                    vec![5, 11],
                    vec![11, 1],
                    vec![5, 13],
                    vec![10, 14],
                    vec![9, 15]
                ]
            ),
            5
        );
        assert_eq!(
            Solution::minimum_score(
                vec![1, 5, 5, 4, 11],
                vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]]
            ),
            9
        );
        assert_eq!(
            Solution::minimum_score(
                vec![5, 5, 2, 4, 4, 2],
                vec![vec![0, 1], vec![1, 2], vec![5, 2], vec![4, 3], vec![1, 3]]
            ),
            0
        );
    }
}
