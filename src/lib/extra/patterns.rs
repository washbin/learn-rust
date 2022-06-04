struct Point {
    x: i32,
    y: i32,
}
pub fn destrucutre_literal_match() {
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}
pub fn advanced_nested_destrucutre() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

pub fn ignore_remiaining() {
    let origin = Point3d { x: 0, y: 0, z: 0 };
    match origin {
        Point3d { x, .. } => println!("x is {}", x),
    }

    let nums = (2, 3, 4, 5, 6, 6);
    match nums {
        (first, .., last) => println!("Some numbers: {}, {}", first, last),
    };
}

pub fn match_guard() {
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

pub fn test_and_save_in_one_pattern() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
