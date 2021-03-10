#[derive(Debug)]
struct Node {
    start: i32,
    end: i32,
    count: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

struct MyCalendarTwo {
    root: Option<Box<Node>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {
    fn new() -> Self {
        Self { root: None }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        if Self::check(&self.root, start, end) {
            Self::update(&mut self.root, start, end);
            true
        } else {
            false
        }
    }

    fn check(node: &Option<Box<Node>>, start: i32, end: i32) -> bool {
        if let Some(n) = node {
            let left = if n.start <= start {
                true
            } else {
                Self::check(&n.left, start, std::cmp::min(end, n.start))
            };
            let right = if end <= n.end {
                true
            } else {
                Self::check(&n.right, std::cmp::max(n.end, start), end)
            };
            let center = if end <= n.start || n.end <= start {
                true
            } else {
                n.count < 2
            };
            left && right && center
        } else {
            true
        }
    }

    fn update(node: &mut Option<Box<Node>>, start: i32, end: i32) {
        if let Some(n) = node {
            let mut start = start;
            let mut end = end;
            if start < n.start {
                Self::update(&mut n.left, start, std::cmp::min(end, n.start));
                if end <= n.start {
                    return;
                }
                start = n.start;
            }
            if n.end < end {
                Self::update(&mut n.right, std::cmp::max(n.end, start), end);
                if n.end <= start {
                    return;
                }
                end = n.end;
            }

            n.count += 1;
            if start != n.start {
                let left = Some(Box::new(Node {
                    start: n.start,
                    end: start,
                    count: 1,
                    left: n.left.take(),
                    right: None,
                }));
                n.left = left;
                n.start = start;
            }
            if end != n.end {
                let right = Some(Box::new(Node {
                    start: end,
                    end: n.end,
                    count: 1,
                    right: n.right.take(),
                    left: None,
                }));
                n.right = right;
                n.end = end;
            }
        } else {
            *node = Some(Box::new(Node {
                start,
                end,
                count: 1,
                left: None,
                right: None,
            }));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0731() {
        let mut obj = MyCalendarTwo::new();
        assert!(obj.book(10, 20));
        assert!(obj.book(50, 60));
        assert!(obj.book(10, 40));
        assert!(!obj.book(5, 15));
        assert!(obj.book(5, 10));
        assert!(obj.book(25, 55));
    }
}
