pub struct Solution {}

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let l = chars.len();
        let mut prev_c = chars.remove(0);
        chars.push(prev_c);
        let mut count = 1;
        let mut compressed = 1;
        while compressed < l {
            let c = chars.remove(0);
            if c == prev_c {
                count += 1;
            } else {
                if 1 < count {
                    for count_c in count.to_string().chars() {
                        chars.push(count_c);
                    }
                }
                chars.push(c);
                prev_c = c;
                count = 1;
            }
            compressed += 1;
        }
        if 1 < count {
            for count_c in count.to_string().chars() {
                chars.push(count_c);
            }
        }

        println!("{:?}", chars);

        chars.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0443() {
        assert_eq!(
            6,
            Solution::compress(&mut vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'])
        );
        assert_eq!(1, Solution::compress(&mut vec!['a']));
        assert_eq!(
            4,
            Solution::compress(&mut vec![
                'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'
            ])
        );
        assert_eq!(
            6,
            Solution::compress(&mut vec!['a', 'a', 'a', 'b', 'b', 'a', 'a'])
        );
        assert_eq!(
            7,
            Solution::compress(&mut vec!['a', 'a', 'b', 'b', 'c', 'c', '1'])
        );
    }
}
