pub struct Solution {}

impl Solution {
    pub fn recurse(
        i: usize,
        p: usize,
        list: &Vec<Vec<usize>>,
        values: &Vec<i32>,
        k: usize,
        count: &mut i32,
    ) -> usize {
        let mut total_sum = values[i] as usize;
        for child in &list[i] {
            if child == &p {
                continue;
            }
            let sum = Self::recurse(*child, i, list, values, k, count);
            if sum % k == 0 {
                *count += 1;
            } else {
                total_sum += sum;
            }
        }
        total_sum
    }

    pub fn max_k_divisible_components(
        n: i32,
        edges: Vec<Vec<i32>>,
        values: Vec<i32>,
        k: i32,
    ) -> i32 {
        let mut list = vec![vec![]; n as usize];
        for edge in edges {
            list[edge[0] as usize].push(edge[1] as usize);
            list[edge[1] as usize].push(edge[0] as usize);
        }

        let mut count = 0;
        let sum = Self::recurse(0, n as usize, &list, &values, k as usize, &mut count);
        if sum % (k as usize) == 0 {
            count + 1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2872() {
        assert_eq!(
            Solution::max_k_divisible_components(
                7,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![1, 3],
                    vec![1, 4],
                    vec![2, 5],
                    vec![2, 6]
                ],
                vec![
                    1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000,
                    1000000000
                ],
                7
            ),
            1
        );
        assert_eq!(
            Solution::max_k_divisible_components(
                5,
                vec![vec![0, 2], vec![1, 2], vec![1, 3], vec![2, 4]],
                vec![1, 8, 1, 4, 4],
                6
            ),
            2
        );
        assert_eq!(
            Solution::max_k_divisible_components(
                7,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![1, 3],
                    vec![1, 4],
                    vec![2, 5],
                    vec![2, 6]
                ],
                vec![3, 0, 6, 1, 5, 2, 1],
                3
            ),
            3
        );
    }
}
