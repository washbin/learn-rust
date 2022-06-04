use std::{
    borrow::Borrow,
    cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
    vec,
};

pub enum NewList<'a> {
    Cons(i32, &'a NewList<'a>),
    Nil,
}
pub fn new_list() {
    use NewList::{Cons, Nil};
    let b = Cons(1, &Nil);
    let c = Cons(2, &b);
    let d = Cons(3, &c);
    let e = Cons(2, &b);
    let f = Cons(4, &c);
}

pub enum RefList {
    Cons(i32, Rc<RefList>),
    Nil,
}
pub fn reference_count() {
    use RefList::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}
pub fn run_cons() {
    let b = Box::new(5);
    println!("b = {b}");

    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}

pub fn deref_box() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("{} and {}", x, y);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let z = *y;
    assert_eq!(5, z);

    println!("x={} and y={}", x, &y);
}

#[derive(Debug)]
pub struct MyBox<T>(T);
impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// DerefMut trait is not implemented for MyBox<T>
// Like deref but for mutable references

pub fn my_box_run() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("{:?} {:?} {:?}", y, *y, &y);

    // *y = *(y.deref())
    // deref coercion
    let m = MyBox::new(String::from("Rust"));

    let hello = |name: &str| println!("Hello, {}!", name);
    let a = &m;
    hello(a);
    hello(&(*m)[..]);
}

struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
pub fn run_custom_drop() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    drop(c);
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

pub trait Messenger {
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}
impl<'a, T: Messenger> LimitTracker<'a, T> {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::Messenger;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(msg));
            two_borrow.push(String::from(msg));

            // self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = super::LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

#[derive(Debug)]
enum RcRefList {
    Cons(Rc<RefCell<i32>>, Rc<RcRefList>),
    Nil,
}
pub fn run_rc_ref_list() {
    use RcRefList::{Cons, Nil};

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

#[derive(Debug)]
enum RefCycleList {
    Cons(i32, RefCell<Rc<RefCycleList>>),
    Nil,
}
use RefCycleList::{Cons, Nil};
impl RefCycleList {
    fn tail(&self) -> Option<&RefCell<Rc<RefCycleList>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
pub fn ref_cycle() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    println!("a next item = {:?}", a.tail());
}

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}
pub fn create_nodes() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    //     let branch = Rc::new(Node {
    //         value: 5,
    //         children: RefCell::new(vec![Rc::clone(&leaf)]),
    //         parent: RefCell::new(Weak::new()),
    //     });
    //     *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    //     println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
