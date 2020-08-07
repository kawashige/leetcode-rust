pub struct Solution {}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() > 0 {
            let mut i: i32 = 0;
            let mut j: i32 = nums.len() as i32 - 1;
            while i <= j {
                let mid = (i + j) / 2;
                if nums[mid as usize] == target {
                    i = mid;
                    j = mid;
                    while i > 0 && nums[i as usize - 1] == target {
                        i -= 1;
                    }
                    while j < nums.len() as i32 - 1 && nums[j as usize + 1] == target {
                        j += 1;
                    }
                    return vec![i as i32, j as i32];
                } else if target < nums[mid as usize] {
                    j = mid - 1;
                } else {
                    i = mid + 1;
                }
            }
        }
        vec![-1, -1]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0034() {
        assert_eq!(
            vec![3, 4],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8)
        );
        assert_eq!(
            vec![-1, -1],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6)
        );
        assert_eq!(vec![-1, -1], Solution::search_range(vec![], 0));
        assert_eq!(vec![1, 1], Solution::search_range(vec![1, 4], 4));
        assert_eq!(vec![-1, -1], Solution::search_range(vec![1], 0));
        assert_eq!(vec![0, 0], Solution::search_range(vec![1], 1));
    }
}
