pub struct Solution {
    bad_version: i32,
}

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut i = 1;
        let mut j = n;
        while i <= j {
            let mid = i + (j - i) / 2;
            if self.isBadVersion(mid) {
                j = mid - 1;
                if !self.isBadVersion(j) {
                    return mid;
                }
            } else {
                i = mid + 1;
                if self.isBadVersion(i) {
                    return i;
                }
            }
        }
        0
    }

    fn isBadVersion(&self, n: i32) -> bool {
        self.bad_version <= n
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0278() {
        let solution = Solution {
            bad_version: 1702766719,
        };
        assert_eq!(1702766719, solution.first_bad_version(2126753390));

        let solution = Solution { bad_version: 4 };
        assert_eq!(4, solution.first_bad_version(5));

        let solution = Solution { bad_version: 1 };
        assert_eq!(1, solution.first_bad_version(1));

        let solution = Solution { bad_version: 1 };
        assert_eq!(1, solution.first_bad_version(5));

        let solution = Solution { bad_version: 5 };
        assert_eq!(5, solution.first_bad_version(5));
    }
}
