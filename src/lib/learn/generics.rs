use std::fmt::Display;

pub fn generics() {
    struct Point<T> {
        x: T,
        y: T,
    }
    impl<T> Point<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
        fn x(&self) -> &T {
            &self.x
        }
    }
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    impl<T: Display + PartialOrd> Point<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    // implement trait for any type that implements another trait.
    // impl of trait on any type that satisfies trait bounds are called
    // blanket implementation,  extensive in standard library

    let integer = Point { x: 5, y: 100 };
    let float = Point { x: 1.0, y: 19.7 };

    struct MultiPoint<T, U> {
        x: T,
        y: U,
    }
    impl<T, U> MultiPoint<T, U> {
        fn mixup<V, W>(self, other: MultiPoint<V, W>) -> MultiPoint<T, W> {
            MultiPoint {
                x: self.x,
                y: other.y,
            }
        }
    }

    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn largest_without_copy<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let num_list = vec![1, 2, 3, 4, 8, 6];
    let char_list = vec!['a', 'c', 'b'];
    println!(
        "The largest char and num are {}, {}",
        largest(&char_list),
        largest(&num_list)
    );
}
