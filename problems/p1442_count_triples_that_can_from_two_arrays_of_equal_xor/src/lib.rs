pub struct Solution {}

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut acc = vec![0; arr.len() + 1];
        acc[1] = arr[0] as usize;
        for i in 1..arr.len() {
            acc[i + 1] = (arr[i] as usize) ^ acc[i];
        }

        let mut count = 0;
        for k in 1..arr.len() {
            for i in 0..k {
                for j in (i + 1)..=k {
                    if acc[j] ^ acc[i] == acc[k + 1] ^ acc[j] {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1442() {
        assert_eq!(Solution::count_triplets(vec![2, 3, 1, 6, 7]), 4);
        assert_eq!(Solution::count_triplets(vec![1, 1, 1, 1, 1]), 10);
    }
}
