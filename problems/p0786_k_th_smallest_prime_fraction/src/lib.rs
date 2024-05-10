pub struct Solution {}

impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut fraction = (0..arr.len())
            .map(|i| {
                (i + 1..arr.len())
                    .map(|j| vec![arr[i], arr[j]])
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();
        fraction.sort_unstable_by(|a, b| (a[0] * b[1]).cmp(&(a[1] * b[0])));
        fraction[k as usize - 1].clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0786() {
        assert_eq!(
            Solution::kth_smallest_prime_fraction(vec![1, 2, 3, 5], 3),
            vec![2, 5]
        );
        assert_eq!(
            Solution::kth_smallest_prime_fraction(vec![1, 7], 1),
            vec![1, 7]
        );
    }
}
