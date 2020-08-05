pub struct Solution {}

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        let mut result = Vec::new();
        let mut i = 0;
        while i < sorted_nums.len() {
            let mut j = i + 1;
            let target_3 = target - sorted_nums[i];
            while j < sorted_nums.len() {
                let mut k = j + 1;
                let target_2 = target_3 - sorted_nums[j];
                while k < sorted_nums.len() {
                    let target_1 = target_2 - sorted_nums[k];
                    for l in (k + 1)..sorted_nums.len() {
                        if sorted_nums[l] == target_1 {
                            result.push(vec![
                                sorted_nums[i],
                                sorted_nums[j],
                                sorted_nums[k],
                                sorted_nums[l],
                            ]);
                            break;
                        } else if sorted_nums[l] > target_1 {
                            break;
                        }
                    }
                    k += 1;
                    while k < sorted_nums.len() && sorted_nums[k - 1] == sorted_nums[k] {
                        k += 1;
                    }
                }
                j += 1;
                while j < sorted_nums.len() && sorted_nums[j - 1] == sorted_nums[j] {
                    j += 1;
                }
            }
            i += 1;
            while i < sorted_nums.len() && sorted_nums[i - 1] == sorted_nums[i] {
                i += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0018() {
        assert_eq!(
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]],
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0)
        );
    }
}
