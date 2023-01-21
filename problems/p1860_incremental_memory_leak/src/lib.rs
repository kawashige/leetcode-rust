pub struct Solution {}

impl Solution {
    pub fn mem_leak(memory1: i32, memory2: i32) -> Vec<i32> {
        let mut result = vec![0, memory1, memory2];
        for i in 1.. {
            let j = if result[1] < result[2] { 2 } else { 1 };
            if result[j] < i {
                result[0] = i;
                return result;
            }
            result[j] -= i;
        }

        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1860() {
        assert_eq!(Solution::mem_leak(2, 2), vec![3, 1, 0]);
        assert_eq!(Solution::mem_leak(8, 11), vec![6, 0, 4]);
    }
}
