pub fn ownership_and_borrow() {
    fn gives_ownership() -> String {
        String::from("Hello")
    }

    fn takes_and_gives_back(a_string: String) -> String {
        a_string
    }

    let mut s = String::from("hello");
    s.push_str(", world!");
    let p = &s;
    println!("{}", p);

    let s1 = gives_ownership();
    let s3 = takes_and_gives_back(s1);
    println!("{}", s3);
}
