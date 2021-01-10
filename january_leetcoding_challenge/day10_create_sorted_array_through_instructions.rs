pub struct Solution {}

impl Solution {
    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        let mut nums = Vec::new();
        let mut min = std::i32::MAX;
        let mut max = std::i32::MIN;
        let mut cost: i64 = 0;

        for n in instructions {
            min = std::cmp::min(min, n);
            max = std::cmp::max(max, n);
            if n == min {
                nums.insert(0, n);
                continue;
            }
            if max == n {
                nums.push(n);
                continue;
            }
            match nums.binary_search(&n) {
                Ok(i) => {
                    let j = (0..=i)
                        .rev()
                        .take_while(|j| nums[*j as usize] == n)
                        .last()
                        .unwrap();
                    let k = (i..nums.len())
                        .take_while(|j| nums[*j as usize] == n)
                        .last()
                        .unwrap();
                    cost += std::cmp::min(j, nums.len() - k - 1) as i64;
                    nums.insert(i, n);
                }
                Err(i) => {
                    cost += std::cmp::min(i, nums.len() - i) as i64;
                    nums.insert(i, n);
                }
            }
        }

        println!("cost: {}", cost);
        (cost % (1_000_000_000 + 7)) as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day10() {
        assert_eq!(0, Solution::create_sorted_array(vec![1]));
        assert_eq!(1, Solution::create_sorted_array(vec![1, 5, 6, 2]));
        assert_eq!(3, Solution::create_sorted_array(vec![1, 2, 3, 6, 5, 4]));
        assert_eq!(
            4,
            Solution::create_sorted_array(vec![1, 3, 3, 3, 2, 4, 2, 1, 2])
        );
        assert_eq!(
            118,
            Solution::create_sorted_array(vec![
                24, 17, 2, 7, 13, 12, 22, 21, 26, 6, 25, 15, 27, 11, 28, 32, 30, 33, 5, 23, 1, 29,
                20, 4, 31, 34, 16, 10, 9, 8, 3, 18, 14, 19
            ])
        );
    }
}
