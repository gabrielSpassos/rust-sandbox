pub fn operators(x: i32) {

    println!("\nIf-else POC!");

    if x > 0 {
        println!("x is positive");
    } else if x < 0 {
        println!("x is negative");
    } else {
        println!("x is zero");
    }

    let is_even = if x % 2 == 0 { true } else { false };
    println!("Is x even? {}", is_even);
}