mod stack;

use stack::Stack;

fn main() {
    println!("Stack POC!");

    let mut stack = Stack::new();

    println!("Pushing...");
    stack.push(10);
    stack.push(20);
    stack.push(30);

    println!("{:?}", stack);
    println!("Stack Length: {:?}", stack.len());

    println!("Pop: {:?}", stack.pop());
    println!("After pop: {:?}", stack);

    while !stack.is_empty() {
        println!("Popped: {:?}", stack.pop());
    }

    println!("Stack Length: {:?}", stack.len());
    println!("Empty: {}", stack.is_empty());
}
