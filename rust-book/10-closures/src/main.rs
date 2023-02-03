fn main() {
    let message = String::from("test");
    let mut list = [1, 2];

    let a = do_something(|| {
        list[1] = 233;
        println!("{}", message);
        223
    });

    list[1] = a;
}

fn do_something<F: FnMut() -> u32>(mut f: F) -> u32 {
    let a = f();
    a
}
