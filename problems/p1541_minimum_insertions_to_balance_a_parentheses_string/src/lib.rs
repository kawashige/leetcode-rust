pub struct Solution {}

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut open = 0;
        let mut close = 0;
        let mut insertions = 0;

        for b in s.as_bytes() {
            if b == &b'(' {
                if 0 < close {
                    if close % 2 == 1 {
                        insertions += 1;
                        close += 1;
                    }
                    let b = (close / 2).min(open);
                    close -= b * 2;
                    open -= b;
                    insertions += close / 2;
                    close = 0;
                }
                open += 1;
            } else {
                close += 1;
            }
        }

        if 0 < close {
            if close % 2 == 1 {
                insertions += 1;
                close += 1;
            }
            let b = (close / 2).min(open);
            close -= b * 2;
            open -= b;
            insertions += close / 2;
        }

        insertions + open * 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1541() {
        assert_eq!(Solution::min_insertions("(()))".to_string()), 1);
        assert_eq!(Solution::min_insertions("())".to_string()), 0);
        assert_eq!(Solution::min_insertions("))())(".to_string()), 3);
    }
}
