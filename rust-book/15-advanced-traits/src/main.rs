use std::ops::Add;

fn main() {
    let millis = Millimeters(120);
    let meters = Meters(2);
    let sum = millis + meters;

    println!("{:?}", sum);

    let millis = Millimeters(120);
    let meters = Meters(2);
    let sum = millis.my_add(meters);

    println!("{:?}", sum);
}

#[derive(Debug)]
struct Millimeters(u32);

#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

trait MyAdd<Rhs=Self> {
    type Output;

    fn my_add(self, rhs: Rhs) -> Self::Output;
}

impl MyAdd<Meters> for Millimeters {
    type Output = Millimeters;

    fn my_add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}
