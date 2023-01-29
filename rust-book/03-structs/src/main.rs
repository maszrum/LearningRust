fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    println!("rect is {:?}", rect);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
