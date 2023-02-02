use std::ops;

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle {
            width,
            height
        }
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    pub fn do_panic(&self) {
        panic!("Intentional panic");
    }
}

impl ops::Add<Rectangle> for Rectangle {
    type Output = Rectangle;

    fn add(self, rhs: Rectangle) -> Self::Output {
        Rectangle {
            width: self.width + rhs.width,
            height: self.height + rhs.height
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };

        let smaller = Rectangle {
            width: 5,
            height: 1
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };

        let smaller = Rectangle {
            width: 5,
            height: 1
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn add_operator_should_add_width_and_height() {
        let lhs = Rectangle {
            width: 8,
            height: 7
        };

        let rhs = Rectangle {
            width: 5,
            height: 1
        };

        let sum = lhs + rhs;

        assert_eq!(sum.width, 13);
        assert_eq!(sum.height, 8);
    }

    #[test]
    #[should_panic(expected = "Intentional panic")]
    fn function_should_panic() {
        let rect = Rectangle {
            width: 12,
            height: 3
        };

        rect.do_panic();
    }
}
