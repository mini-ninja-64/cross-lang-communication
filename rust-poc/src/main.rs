use rust_poc::{library_function, print_simple_pair, simple_pair, LIBRARY_ENUM_HIGH};

fn main() {
    unsafe {
        library_function(1, LIBRARY_ENUM_HIGH);
    }

    let pair = simple_pair { a: 10, b: 20 };
    unsafe {
        print_simple_pair(&pair);
    }
}
