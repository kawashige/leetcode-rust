struct Robot {
    width: i32,
    height: i32,
    pos: Vec<i32>,
    dir: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Robot {
    fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            pos: vec![0, 0],
            dir: 0,
        }
    }

    fn step(&mut self, num: i32) {
        let mut num = num % ((self.width + self.height) * 2 - 4);
        if self.pos == vec![0, 0] && num == 0 {
            self.dir = 3;
        }
        while 0 < num {
            match self.dir {
                0 => {
                    self.pos[0] += num;
                    if self.width <= self.pos[0] {
                        num = self.pos[0] + 1 - self.width;
                        self.pos[0] = self.width - 1;
                        self.dir = 1;
                    } else {
                        num = 0;
                    }
                }
                1 => {
                    self.pos[1] += num;
                    if self.height <= self.pos[1] {
                        num = self.pos[1] + 1 - self.height;
                        self.pos[1] = self.height - 1;
                        self.dir = 2;
                    } else {
                        num = 0;
                    }
                }
                2 => {
                    self.pos[0] -= num;
                    if self.pos[0] < 0 {
                        num = self.pos[0].abs();
                        self.pos[0] = 0;
                        self.dir = 3;
                    } else {
                        num = 0;
                    }
                }
                3 => {
                    self.pos[1] -= num;
                    if self.pos[1] < 0 {
                        num = self.pos[1].abs();
                        self.pos[1] = 0;
                        self.dir = 0;
                    } else {
                        num = 0;
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    fn get_pos(&self) -> Vec<i32> {
        self.pos.clone()
    }

    fn get_dir(&self) -> String {
        ["East", "North", "West", "South"][self.dir].to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2069() {
        let mut obj = Robot::new(97, 98);
        obj.step(66392);
        obj.step(83376);
        obj.step(71796);
        obj.step(57514);
        obj.step(36284);
        obj.step(69866);
        obj.step(31652);
        println!("{:?}", obj.get_pos());
        println!("{:?}", obj.get_dir());
        obj.step(32038);
        assert_eq!(obj.get_pos(), vec![0, 0]);
        assert_eq!(obj.get_dir(), "South".to_string());

        let mut obj = Robot::new(6, 3);
        obj.step(2);
        obj.step(2);
        assert_eq!(obj.get_pos(), vec![4, 0]);
        assert_eq!(obj.get_dir(), "East".to_string());
        obj.step(2);
        obj.step(1);
        obj.step(4);
        assert_eq!(obj.get_pos(), vec![1, 2]);
        assert_eq!(obj.get_dir(), "West".to_string());
    }
}
