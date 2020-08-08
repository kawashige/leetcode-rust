pub struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn recuresive_permute(nums: &mut [i32], heads: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            if nums.len() == 0 {
                return heads.clone();
            }
            let mut result = Vec::new();
            for i in 0..nums.len() {
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
        recuresive_permute(&mut nums.clone(), &vec![])
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0046() {
        let result = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        assert_eq!(result, Solution::permute(vec![1, 2, 3]));
    }
}
