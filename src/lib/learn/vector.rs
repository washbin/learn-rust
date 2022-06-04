pub fn vectors() {
    let mut v: Vec<i32> = Vec::new();
    let new_vec = vec![1, 2, 3, 4];
    v.push(3);
    v.push(5);
    let first = &v[0];
    // v.push(8);
    println!("The first element is : {}", first);
    println!("Third element of new_vec is {}", &new_vec[2]);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
    // panics
    // let does_not_exist = &v[100];

    // does not panic
    // let does_not_exist = v.get(100);

    for i in &new_vec {
        println!("{}", i)
    }

    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.23),
        SpreadSheetCell::Text(String::from("blue")),
    ];
    for i in &row {
        println!("{:?}", i);
    }
}
