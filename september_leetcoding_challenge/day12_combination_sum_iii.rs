pub struct Solution {}

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        let mut i: i32 = 1;
        while i < (1 << 9) {
            if i.count_ones() == k as u32 {
                let nums = (1..10).fold(Vec::new(), |mut acc, x| {
                    if (i & (1 << (x - 1))).count_ones() == 1 {
                        acc.push(x);
                    }
                    acc
                });
                if nums.iter().sum::<i32>() == n {
                    results.push(nums)
                }
            }
            i += 1;
        }
        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day12() {
        assert_eq!(vec![] as Vec<Vec<i32>>, Solution::combination_sum3(3, 25));
        assert_eq!(vec![vec![1, 2, 4]], Solution::combination_sum3(3, 7));
        assert_eq!(
            vec![vec![2, 3, 4], vec![1, 3, 5], vec![1, 2, 6]],
            Solution::combination_sum3(3, 9)
        );
        assert_eq!(vec![] as Vec<Vec<i32>>, Solution::combination_sum3(2, 18));
    }
}
