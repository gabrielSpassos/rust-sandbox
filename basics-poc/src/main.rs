mod operators;
mod tuples;
mod arrays;
mod structs;

fn main() {
    primitives();
    operators::operators();
    tuples::tuples();
    arrays::arrays();
    structs::structs();
}

fn primitives() {
    println!("Primitives POC!");

    let a: i8 = 127;
    let b: i16 = 32767;
    let c: i32 = 2147483647;
    let d: i64 = 9223372036854775807;
    let e: i128 = 170141183460469231731687303715884105727;
    let f: isize = 9223372036854775807;
    println!("Signed integers a: {}, b: {}, c: {}, d: {}, e: {}, f: {}", a, b, c, d, e, f);

    let a: u8 = 255;
    let b: u16 = 65535;
    let c: u32 = 4294967295;
    let d: u64 = 18446744073709551615;
    let e: u128 = 340282366920938463463374607431768211455;
    let f: usize = 18446744073709551615;
    println!("Unsigned integers a: {}, b: {}, c: {}, d: {}, e: {}, f: {}", a, b, c, d, e, f);

    let a: f32 = 3.14;
    let b: f64 = 3.141592653589793;
    println!("Floating-point numbers a: {}, b: {}", a, b);

    let a: char = 'a';
    let b: char = 'α';
    let c: char = '😀';
    let d: char = '𐍈';
    let e: char = '∞';
    println!("Characters a: {}, b: {}, c: {}, d: {}, e: {}", a, b, c, d, e);

    let a: bool = true;
    let b: bool = false;
    println!("Booleans a: {}, b: {}", a, b);

    let a: &str = "Hello, Rust!";
    let b: String = String::from("Hello, Rust!");
    println!("Strings a: {}, b: {}", a, b);

    let a: [i32; 3] = [1, 2, 3];
    let b: [f64; 2] = [3.14, 2.718];
    println!("Arrays a: {:?}, b: {:?}", a, b);

    let a: (i32, f64, char) = (42, 3.14, 'a');
    println!("Tuple a: {:?}", a);
}