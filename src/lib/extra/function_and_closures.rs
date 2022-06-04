// fn type - function pointer
// not to be confused with Fn closure trait
//
// fn type implements all Fn tratis Fn FnMut FnOnce

fn add_one(x: i32) -> i32 {
    x + 1
}

fn double_answer(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn run() {
    let answer = double_answer(add_one, 5);
    println!("The answer is: {}", answer);

    // using closures
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    // using regular functions
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    // Tuple strcut haxx
    enum Status {
        Value(u32),
        Stop,
    }

    let a = 0u32..20;
    let list_of_statuses: Vec<Status> = (0..20).map(Status::Value).collect();
}

fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn return_closure_another() -> impl Fn(i32) -> i32 {
    |x| x + 1
}
