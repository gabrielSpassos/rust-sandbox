use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

pub fn my_from() {
    let s = "hello".to_string();
    let s2: String = s.into();
    println!("{}", s2);

    let num = Number::from(30);
    let value = num.value;
    println!("My number is {:?}, value is {}", num, value);
}