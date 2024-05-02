pub struct Solution {}

impl Solution {
    pub fn is_ok(mid: i32, k: i32, price: &[i32]) -> bool {
        let mut cur = price[0];
        let mut count = 1;
        let mut i = 1;

        while i < price.len() {
            while i < price.len() && price[i] < cur + mid {
                i += 1;
            }
            if i < price.len() {
                count += 1;
                cur = price[i];
                i += 1;
            }
        }

        k <= count
    }

    pub fn maximum_tastiness(price: Vec<i32>, k: i32) -> i32 {
        let mut price = price;
        price.sort_unstable();

        let mut ok = 0;
        let mut ng = *price.last().unwrap();

        while ok + 1 < ng {
            let mid = (ok + ng) / 2;
            if Self::is_ok(mid, k, &price) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2517() {
        assert_eq!(Solution::maximum_tastiness(vec![13, 5, 1, 8, 21, 2], 3), 8);
        assert_eq!(Solution::maximum_tastiness(vec![1, 3, 1], 2), 2);
        assert_eq!(Solution::maximum_tastiness(vec![7, 7, 7, 7], 2), 0);
    }
}
