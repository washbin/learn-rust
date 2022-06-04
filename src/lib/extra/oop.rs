// Object-oriented programs are made up of objects. An object packages both data and the procedures that operate on that data. The procedures are typically called methods or operations
//
//
// Encapsulation hides implemenentation details
// sturcts, methods priavte public
//
// Inheritance allows one class to inherit the properties of another
// as type system and code sharing
// achievable with traits for code sharing
//
// Polymorphism allows an object to take on many forms
// code that can work with multiple data types

trait Hello {
    fn hello(&self);

    fn hi(&self) {
        println!("hi");
    }
}

struct A {
    a: i32,
}
impl Hello for A {
    fn hello(&self) {
        println!("hello");
    }
}

pub fn run() {
    let a = A { a: 1 };
    a.hello();
    a.hi();
}
