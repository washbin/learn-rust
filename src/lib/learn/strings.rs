pub fn strings() {
    let s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let mut s = String::from("initial contents");
    s.push_str(" and more");
    s.push('.');
    let s1 = String::from("Hello, ");
    let s2 = String::from("Wrold!");
    let s3 = s1 + &s2; // s1.add(&s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s_does_not_take_ownership = format!("{}-{}-{}", s1, s2, s3);
    let s_takes_ownership = s1 + "-" + &s2 + "-" + &s3;

    // bytes => Scalar Values => Grapheme Clusters
    for c in "🦀🐍".chars() {
        println!("{}", c);
    }

    for c in "🦀🐍".bytes() {
        println!("{}", c);
    }
}
