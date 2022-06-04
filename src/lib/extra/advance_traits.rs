// Associated Types
trait MyIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// impl MyIterator for Someithing {
//     type Item = u32;
//     fn next(&mut self) -> Option<Self::Item> {}
// }

trait GenericIteratorImpl<T> {
    fn next(&mut self) -> Option<T>;
}

// Default Generic Type Parameters
// Operator Overloading
use std::ops::{Add, Deref};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn verify() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    )
}

trait ShowDefaultTypeParameter<Iam = Self> {
    type Output;

    fn something(self, iam: Iam) -> Self::Output;
}

// NewTypePattern => wrapping of existing typ ein another struct

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

// Fully Qualified syntax, Disambiguation
trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.")
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Oh heavens! Take me up into your path.")
    }
}
impl Human {
    fn fly(&self) {
        println!("Fast fast, wave hand fast.")
    }
}

pub fn call_with_disambuiguation() {
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
    Human::fly(&person);
}

trait Animal {
    fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    // Associated functions
    fn baby_name() -> String {
        String::from("Puppy")
    }
}

pub fn call_dog() {
    println!("A baby dog is called a {}", Dog::baby_name())
}

pub fn amibigious_call_animal() {
    // println!("A baby dog is called a {}", Animal::baby_name())
    // Disambiguate -> Full qualified syntax
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name())
}

// SuperTraits
use std::fmt::{self, write};

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

#[derive(Debug)]
struct MyPoint {
    x: i32,
    y: i32,
}
impl OutlinePrint for MyPoint {}

impl fmt::Display for MyPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// NewType Pattern
// Newtype is a term originates from Haskell
//
// Implementing Diplay on Vec<T>
// Getting around the implications for implementing external type on external trait with newtype
// pattern

struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
impl Deref for Wrapper {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn testing_other_methods_of_vec_as_deref_is_implemented() {
    let a = Wrapper(vec![String::from("hello"), String::from("World")]);
    println!("{}", a)
}
