struct LUPrefix {
    prefix: i32,
    videos: Vec<bool>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LUPrefix {
    fn new(n: i32) -> Self {
        let mut videos = vec![false; n as usize + 1];
        videos[0] = true;
        Self { prefix: 0, videos }
    }

    fn upload(&mut self, video: i32) {
        self.videos[video as usize] = true;
        while self.prefix as usize + 1 < self.videos.len() && self.videos[self.prefix as usize + 1]
        {
            self.prefix += 1;
        }
    }

    fn longest(&self) -> i32 {
        self.prefix
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2424() {
        let mut server = LUPrefix::new(4);
        server.upload(3);
        assert_eq!(server.longest(), 0);
        server.upload(1);
        assert_eq!(server.longest(), 1);
        server.upload(2);
        assert_eq!(server.longest(), 3);
    }
}
