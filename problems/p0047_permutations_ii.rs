pub struct Solution {}

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn recuresive_permute(nums: &mut [i32], heads: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            if nums.len() == 0 {
                return heads.clone();
            }
            let mut result = Vec::new();
            for i in 0..nums.len() {
                if i > 0 && nums[i - 1] == nums[i] {
                    continue;
                }
                let mut new_heads = Vec::new();
                for h in heads {
                    new_heads.push(vec![h.clone(), vec![nums[i]]].concat());
                }
                if new_heads.len() == 0 {
                    new_heads.push(vec![nums[i]]);
                }
                result.append(&mut recuresive_permute(
                    &mut [&nums[0..i], &nums[(i + 1)..nums.len()]].concat(),
                    &new_heads,
                ));
            }
            result
        }
        let mut nums_sorted = nums.clone();
        nums_sorted.sort();
        recuresive_permute(&mut nums_sorted, &vec![])
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0047() {
        let result = vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]];
        assert_eq!(result, Solution::permute_unique(vec![1, 1, 2]));
        assert_eq!(vec![vec![1, 1, 1]], Solution::permute_unique(vec![1, 1, 1]));

        let result2 = vec![
            vec![0, 3, 3, 3],
            vec![3, 0, 3, 3],
            vec![3, 3, 0, 3],
            vec![3, 3, 3, 0],
        ];
        assert_eq!(result2, Solution::permute_unique(vec![3, 3, 0, 3]));
    }
}
