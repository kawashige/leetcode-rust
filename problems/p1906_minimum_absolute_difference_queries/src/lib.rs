pub struct Solution {}

impl Solution {
    pub fn min_difference(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut count = vec![[0; 101]; nums.len() + 1];
        for i in 0..nums.len() {
            count[i + 1] = count[i].clone();
            count[i + 1][nums[i] as usize] += 1;
        }

        queries
            .into_iter()
            .map(|q| {
                let mut prev = 0;
                let mut min = std::usize::MAX;
                for i in 1..count[0].len() {
                    if count[q[0] as usize][i] < count[q[1] as usize + 1][i] {
                        if prev != 0 {
                            min = min.min(i - prev);
                        }
                        prev = i;
                    }
                }
                if min == std::usize::MAX {
                    -1
                } else {
                    min as i32
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1906() {
        assert_eq!(
            Solution::min_difference(
                vec![1, 3, 4, 8],
                vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![0, 3]]
            ),
            vec![2, 1, 4, 1]
        );
        assert_eq!(
            Solution::min_difference(
                vec![4, 5, 2, 2, 7, 10],
                vec![vec![2, 3], vec![0, 2], vec![0, 5], vec![3, 5]]
            ),
            vec![-1, 1, 1, 3]
        );
    }
}
