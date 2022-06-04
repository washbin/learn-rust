// trait objects
pub trait Draw {
    fn draw(&self);
}

// duck typing - if it walks like a duck and quacks like a duck, it must be a duck
//
// dyanmic dispatch / static dispatch
//
//
// object safe traits into trait objects
//
// trait object safe if :-
// - return type isn't Self
// - There are no generic type parameters

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!("Drawing button {}", self.label);
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {}
}

pub fn actual_drawing() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

mod trait_bound {
    use super::Draw;

    pub struct Screen<T: Draw> {
        pub components: Vec<T>,
    }

    impl<T: Draw> Screen<T> {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
}
