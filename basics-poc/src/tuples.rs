pub fn tuples() {
    println!("\nTuples POC!");

    let pair = (42, true);
    println!("Original pair: {:?}", pair);

    let reversed_pair = reverse(pair);
    println!("Reversed pair: {:?}", reversed_pair);

    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);
    println!("Long tuple third value: {}", long_tuple.2);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("destructured values: {:?}, {:?}, {:?}, {:?}", a, b, c, d);
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables.
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}
