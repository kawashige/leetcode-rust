pub struct Solution {}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut list: Vec<Vec<i32>> = Vec::new();
        for i in 0..nums.len() {
            if list.is_empty() {
                list.push(vec![nums[i]]);
                continue;
            }

            let mut smallest = true;
            let mut new_list = Vec::new();
            for j in 0..list.len() {
                if list[j].contains(&nums[i]) {
                    continue;
                } else if list[j].last().unwrap() < &nums[i] {
                    smallest = false;
                    list[j].push(nums[i]);
                } else if list[j][0] < nums[i] {
                    smallest = false;
                    new_list.push(
                        list[j]
                            .iter()
                            .filter(|l| *l < &nums[i])
                            .cloned()
                            .chain(std::iter::once(nums[i]))
                            .collect::<Vec<i32>>(),
                    );
                }
            }
            if smallest {
                list.push(vec![nums[i]]);
            } else {
                list.append(&mut new_list);
            }
        }
        list.into_iter().map(|l| l.len() as i32).max().unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0300() {
        assert_eq!(
            6,
            Solution::length_of_lis(vec![3, 5, 6, 2, 5, 4, 19, 5, 6, 7, 12])
        );
        assert_eq!(4, Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]));
        assert_eq!(
            4,
            Solution::length_of_lis(vec![10, 100, 11, 101, 1, 12, 2, 3, 14])
        );
        assert_eq!(1, Solution::length_of_lis(vec![10]));
        assert_eq!(1, Solution::length_of_lis(vec![10, 9]));
        assert_eq!(0, Solution::length_of_lis(vec![]));
    }
}
