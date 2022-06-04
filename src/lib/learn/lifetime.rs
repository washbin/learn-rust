use std::fmt::Display;

// generic type paramenters, traits, trait bounds, generic lifetime parameters
pub fn generic_lifetime_traits() {
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    longest_with_an_announcement("abc", "defg", "HELLO");
}

pub fn lifetimes() {
    let r;
    {
        let x = 5;
        // r = &x;
        r = x;
    }
    println!("r: {}", r);

    let strring1 = String::from("abcd");
    let string2 = "xyz";
    println!(
        "The longest string is {}",
        longest(strring1.as_str(), string2)
    );

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // Error: lifetime mismatch
    //     let str1 = String::from("long string is long");
    //     let result;
    //     {
    //         let str2 = String::from("xyz");
    //         result = longest(str1.as_str(), str2.as_str());
    //     }
    //     println!("The longest string is {}", result);

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }

        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // lifetime ellision rules
    // parameters -> input lifetimes, return -> output lifetimes
    // three rules
    // each referenced parmeter gets its own lifetime parameter
    // if there is only one input lifetime, it is assigned to all output lifetimes
    // multiple input lifetimes, one is &self or &mut self, the lifetime of self is assigned to all
    // output lifetime parameters
    //
    // &str have static lifetimes, entire duration of the program
}
