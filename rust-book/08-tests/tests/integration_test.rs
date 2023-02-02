use tests::Rectangle;

#[test]
fn it_adds_two() {
    let lhs = Rectangle::new(10, 11);
    let rhs = Rectangle::new(1, 5);

    let sum = lhs + rhs;
    assert_eq!(sum.width, 11);
    assert_eq!(sum.height, 16);
}
