pub struct Solution {}

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        fn find(n: i32, codes: Vec<i32>, bits: &Vec<i32>) -> Option<Vec<i32>> {
            if codes.len() == n as usize {
                return Some(codes);
            }

            let last_val = codes.last().unwrap();
            for b in bits {
                if !codes.contains(&(b ^ last_val)) {
                    let mut new_codes = codes.clone();
                    new_codes.push(b ^ last_val);
                    match find(n, new_codes, &bits) {
                        Some(v) => return Some(v),
                        None => {}
                    }
                }
            }
            None
        }
        match n {
            0 => vec![0],
            _ => find(
                (2 as i32).pow(n as u32),
                vec![0],
                &((0..n).map(|i| 1 << i).collect()),
            )
            .unwrap(),
        }
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
