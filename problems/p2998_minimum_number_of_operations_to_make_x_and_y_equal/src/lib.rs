use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn recurse(
        x: i32,
        y: i32,
        operations: i32,
        min_operaitons: &mut i32,
        memo: &mut HashMap<(i32, i32), i32>,
    ) {
        if x == y {
            if operations < *min_operaitons {
                *min_operaitons = operations;
            }
            return;
        }
        if *min_operaitons <= operations {
            return;
        }
        if let Some(op) = memo.get(&(x, y)) {
            if op <= &operations {
                return;
            }
        }
        memo.insert((x, y), operations);
        if x < y {
            Self::recurse(x + 1, y, operations + 1, min_operaitons, memo);
        } else {
            if x < 11000 {
                Self::recurse(x + 1, y, operations + 1, min_operaitons, memo);
            }
            if x % 11 == 0 {
                Self::recurse(x / 11, y, operations + 1, min_operaitons, memo);
            }
            if x % 5 == 0 {
                Self::recurse(x / 5, y, operations + 1, min_operaitons, memo);
            }
            if 0 < x {
                Self::recurse(x - 1, y, operations + 1, min_operaitons, memo);
            }
        }
    }

    pub fn minimum_operations_to_make_equal(x: i32, y: i32) -> i32 {
        let mut min_operations = (x - y).abs();
        Self::recurse(x, y, 0, &mut min_operations, &mut HashMap::new());
        min_operations
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2998() {
        assert_eq!(Solution::minimum_operations_to_make_equal(89, 57), 32);
        assert_eq!(Solution::minimum_operations_to_make_equal(26, 1), 3);
        assert_eq!(Solution::minimum_operations_to_make_equal(54, 2), 4);
        assert_eq!(Solution::minimum_operations_to_make_equal(25, 30), 5);
    }
}
