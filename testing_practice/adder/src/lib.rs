#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn large_can_hold_small() {
        let larger = Rectangle { width: 10, height: 12 };
        let smaller = Rectangle { width: 8, height: 7 };
        assert!(larger.can_hold(&smaller));
    }

    fn small_cannot_hold_large() {
        let larger = Rectangle { width: 10, height: 12 };
        let smaller = Rectangle { width: 8, height: 7 };
        assert!(!smaller.can_hold(&larger));
    }
}
