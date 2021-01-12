pub struct Solution {}

impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a == b {
            -1
        } else if a.len() < b.len() {
            b.len() as i32
        } else {
            a.len() as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0521() {
        assert_eq!(
            3,
            Solution::find_lu_slength("aba".to_string(), "cdc".to_string())
        );
        assert_eq!(
            3,
            Solution::find_lu_slength("aaa".to_string(), "vvv".to_string())
        );
        assert_eq!(
            -1,
            Solution::find_lu_slength("aaa".to_string(), "aaa".to_string())
        );
    }
}
