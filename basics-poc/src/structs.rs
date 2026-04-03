#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

pub fn structs() {
    println!("\nStructs POC!");

    let name = String::from("Gabriel");
    let age = 28;
    let gabriel = Person { name, age };
    println!("person {:?}", gabriel);

    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };
    let bottom_right = Point { x: 10.3, ..another_point };
    let Point { x: left_edge, y: top_edge } = point;

    println!("point coordinates: ({}, {})", point.x, point.y);
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };
    println!("rectangle {:?}", _rectangle);

    let _unit = Unit;


    let pair = Pair(1, 0.1);
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    println!("pair contains {:?} and {:?}", integer, decimal);
}