pub struct Solution {}

impl Solution {
    pub fn minimum_index(capacity: Vec<i32>, item_size: i32) -> i32 {
        let mut result = -1;
        for i in 0..capacity.len() {
            if item_size <= capacity[i] && (result == -1 || capacity[i] < capacity[result as usize])
            {
                result = i as i32;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3861() {
        assert_eq!(Solution::minimum_index(vec![1, 5, 3, 7], 3), 2);
        assert_eq!(Solution::minimum_index(vec![3, 5, 4, 3], 2), 0);
        assert_eq!(Solution::minimum_index(vec![4], 5), -1);
    }
}

fn main() {}
