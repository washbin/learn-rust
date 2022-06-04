pub fn structs() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn change_height(&mut self, new_height: u32) {
            self.height = new_height;
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    #[allow(dead_code)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    #[allow(dead_code)]
    impl User {
        fn build_user(email: String, username: String) -> User {
            User {
                email,
                username,
                active: true,
                sign_in_count: 1,
            }
        }
    }

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someuser"),
        active: true,
        sign_in_count: 1,
    };
    user1.username = String::from("anotheruser");

    struct Color(i32, i32, i32);
    let _black = Color(0, 0, 0);

    let mut rect1 = Rectangle {
        width: 40,
        height: 30,
    };
    println!("Area of rectangle 1 is {} square", rect1.area());
    let rect2 = Rectangle::square(32);
    rect1.change_height(35);
    println!(
        "Rectange 2 can hold 1 ? {}",
        Rectangle::can_hold(&rect2, &rect1)
    );

    println!("Rectange 1 can hold 2 ? {}", rect1.can_hold(&rect2));
}
