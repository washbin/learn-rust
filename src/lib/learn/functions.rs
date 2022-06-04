pub fn functions() {
    fn another_function() {
        println!("Another function.");
    }

    fn five() -> i32 {
        5
    }

    fn plus_one(x: i32) -> i32 {
        x + 1
    }

    another_function();
    let five = five();
    let _six = plus_one(five);
    {
        let s = "hello";
        let _b = s;
        println!("{}", s);
    }
}
