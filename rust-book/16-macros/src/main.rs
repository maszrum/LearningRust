// declarative macro
#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let vector_from_own_macro = my_vec![1, 2, 3, 4];
    println!("{:?}", vector_from_own_macro);
}
