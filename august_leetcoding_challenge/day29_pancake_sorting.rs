pub struct Solution {}

impl Solution {
    pub fn pancake_sort(a: Vec<i32>) -> Vec<i32> {
        let l = a.len() as i32;
        let mut nums = a;
        let mut results = Vec::new();
        for target in (2..=l).rev() {
            for i in 0..(nums.len() - 1) {
                println!("{:?},{:?},{:?}", target, i, nums[i]);
                if nums[i] == target {
                    results.push((i + 1) as i32);
                    results.push(nums.len() as i32);
                    let tmp = nums.clone();
                    let mut reversed = tmp[..(i + 1)]
                        .to_vec()
                        .into_iter()
                        .rev()
                        .chain(tmp[(i + 1)..].to_vec().into_iter())
                        .collect::<Vec<i32>>();
                    reversed.reverse();
                    nums = reversed;
                }
            }
            nums.remove(nums.len() - 1);
        }
        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day29() {
        assert_eq!(vec![] as Vec<i32>, Solution::pancake_sort(vec![1, 2, 3]));
        assert_eq!(
            vec![3, 4, 2, 3, 1, 2],
            Solution::pancake_sort(vec![3, 2, 4, 1])
        );
        assert_eq!(
            vec![1, 10, 1, 9, 5, 8, 1, 7, 2, 6, 3, 5, 3, 4, 2, 3],
            Solution::pancake_sort(vec![10, 5, 1, 6, 3, 8, 2, 4, 7, 9])
        );
    }
}
