static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

pub fn constants() {
    let n = 9;

    println!("\nConstants POC");
    println!("This is {}", LANGUAGE);
    println!("The threshold is {} and {} is {} than the threshold", 
        THRESHOLD, n, if is_big(n) { "bigger" } else { "smaller" });
}

fn is_big(x: i32) -> bool {
    x > THRESHOLD
}