use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

// Define `From`
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

pub fn my_from_into_interchange() {
    let int = 5;
    // use `Into`
    let num: Number = int.into();
    println!("from_into: My number is {:?}, value is {}", num, num.value);
}