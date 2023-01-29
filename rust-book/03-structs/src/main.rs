fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    println!("rect is {:?}", rect);
    println!("area is {}", rect.get_area());
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

trait Shape {
    fn get_area(&self) -> u32;
}

impl Shape for Rectangle {
    fn get_area(&self) -> u32 {
        self.width * self.height
    }
}
