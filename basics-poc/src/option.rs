pub fn operators() {
    println!("\nOption POC!");

    let number = Some(7);

    match number {
        Some(i) => println!("Matched {:?}!", i),
        _ => {},
    };


    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }
}