use std::fmt::{Debug, Display};

pub fn traits() {
    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }

        fn summarize_author(&self) -> String {
            format!("(Read more from {}...)", self.summarize())
        }
    }

    struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    // sugar syntax for trait bound
    fn notifty(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    // trait bound syntax
    fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize())
    }
    // multi bounds
    fn noti(item: &(impl Summary + Display)) {}
    fn not<T: Summary + Display>(item: &T) {}
    fn som_fun<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
        1
    }
    // alternate syntax for trait bounds
    fn some_fun<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        1
    }

    // return type implementing traits
    fn returns_summmarizable() -> impl Summary {
        Tweet {
            username: "horse".to_string(),
            content: "of course".to_string(),
            reply: false,
            retweet: false,
        }
    }
}
