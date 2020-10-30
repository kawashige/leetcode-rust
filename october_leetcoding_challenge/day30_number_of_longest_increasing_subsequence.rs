pub struct Solution {}

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }

        let mut length = vec![1];
        let mut counts = vec![1];

        for i in 1..nums.len() {
            let mut l = 1;
            let mut c = 1;
            for j in (0..i).rev() {
                if nums[j] < nums[i] && l <= length[j] + 1 {
                    if l < length[j] + 1 {
                        l = length[j] + 1;
                        c = counts[j];
                    } else {
                        c += counts[j];
                    }
                }
            }
            length.push(l);
            counts.push(c);
        }

        let max = length.iter().max().unwrap();
        (0..length.len())
            .filter(|i| &length[*i] == max)
            .map(|i| counts[i])
            .sum::<i32>()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day30() {
        assert_eq!(
            3,
            Solution::find_number_of_lis(vec![1, 2, 4, 3, 5, 4, 7, 2])
        );
        assert_eq!(0, Solution::find_number_of_lis(vec![]));
        assert_eq!(1, Solution::find_number_of_lis(vec![1]));
        assert_eq!(2, Solution::find_number_of_lis(vec![1, 1]));
        assert_eq!(2, Solution::find_number_of_lis(vec![1, 3, 5, 4, 7]));
        assert_eq!(5, Solution::find_number_of_lis(vec![2, 2, 2, 2, 2]));
    }
}
