pub struct Solution {}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        for x in nums {
            match stack.binary_search(&x) {
                Ok(_) => {}
                Err(i) => {
                    if stack.len() <= i {
                        stack.push(x);
                    } else {
                        stack[i] = x;
                    }
                }
            }
        }

        stack.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day09() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    }
}
