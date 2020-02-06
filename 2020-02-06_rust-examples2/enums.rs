#[derive(Debug)]
enum StringOrInteger {
    S(String),
    I(i32),
}

fn function_that_returns_string_or_integer(x: i32) -> StringOrInteger {
    if x > 5 {
        StringOrInteger::S("one".to_string())
    } else {
        StringOrInteger::I(1)
    }
}

fn main() {
    let x_string_or_int = function_that_returns_string_or_integer(5);

    dbg!(x_string_or_int);
    // println!("{:?}", x_string_or_int);    
}
