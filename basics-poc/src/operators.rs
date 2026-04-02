pub fn operators() {
    println!("Operators POC!");

    println!("Addition [5 + 3]: {}", 5 + 3);
    println!("Subtraction [5 - 3]: {}", 5 - 3);
    println!("Multiplication [5 * 3]: {}", 5 * 3);
    println!("Division [5 / 3]: {}", 5 / 3);
    println!("Remainder [5 % 3]: {}", 5 % 3);
    println!("Exponentiation [5 ^ 3]: {}", 5i32.pow(3));

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
