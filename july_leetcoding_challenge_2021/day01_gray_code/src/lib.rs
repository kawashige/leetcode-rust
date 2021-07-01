pub struct Solution {}

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        if n == 0 {
            return vec![0];
        }

        let l = 2_usize.pow(n as u32);
        let mut result = vec![0_i32];
        let mut used = vec![false; l];
        used[0] = true;

        while result.len() < l {
            if let Some(x) = (0..n)
                .map(|i| (result.last().unwrap() ^ 1 << i) as usize)
                .find(|x| !used[*x])
            {
                used[x] = true;
                result.push(x as i32);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0089() {
        assert_eq!(vec![0, 1, 3, 2], Solution::gray_code(2));
        assert_eq!(vec![0, 1, 3, 2, 6, 7, 5, 4], Solution::gray_code(3));
        assert_eq!(vec![0], Solution::gray_code(0));
    }
}
