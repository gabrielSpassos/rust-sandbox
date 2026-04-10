use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self }
    }
}

pub fn my_into() {
    let int = 5;
    // Try removing the type annotation
    let num: Number = int.into();
    println!("into: My number is {:?}, value is {}", num, num.value);
}