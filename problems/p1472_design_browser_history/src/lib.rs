#[derive(Default)]
struct BrowserHistory {
    history: Vec<String>,
    index: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {
    fn new(homepage: String) -> Self {
        let history = vec![homepage];
        Self { history, index: 0 }
    }

    fn visit(&mut self, url: String) {
        self.history.drain(self.index + 1..);
        self.history.push(url);
        self.index += 1;
    }

    fn back(&mut self, steps: i32) -> String {
        self.index = self.index.saturating_sub(steps as usize);
        self.history[self.index].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        self.index = (self.index + steps as usize).min(self.history.len() - 1);
        self.history[self.index].clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1472() {
        let mut obj = BrowserHistory::new("leetcode.com".to_string());
        obj.visit("google.com".to_string());
        obj.visit("facebook.com".to_string());
        obj.visit("youtube.com".to_string());
        assert_eq!(obj.back(1), "facebook.com".to_string());
        assert_eq!(obj.back(1), "google.com".to_string());
        assert_eq!(obj.forward(1), "facebook.com".to_string());
        obj.visit("linkedin.com".to_string());
        assert_eq!(obj.forward(2), "linkedin.com".to_string());
        assert_eq!(obj.back(2), "google.com".to_string());
        assert_eq!(obj.back(7), "leetcode.com".to_string());
    }
}
