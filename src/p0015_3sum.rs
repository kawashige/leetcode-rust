pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();
        let mut i = 0;
        while i < sorted_nums.len() {
            if sorted_nums[i] > 0 {
                break;
            }
            let mut j = i + 1;
            while j < sorted_nums.len() {
                let find_num = -1 * (sorted_nums[i] + sorted_nums[j]);
                for k in (j + 1)..sorted_nums.len() {
                    if sorted_nums[k] == find_num {
                        results.push(vec![sorted_nums[i], sorted_nums[j], sorted_nums[k]]);
                        break;
                    } else if sorted_nums[k] > find_num {
                        break;
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
        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0015() {
        assert_eq!(
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])
        );
        assert_eq!(vec![vec![0, 0, 0]], Solution::three_sum(vec![0, 0, 0, 0]));
    }
}
