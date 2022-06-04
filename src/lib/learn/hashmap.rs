use std::collections::HashMap;

pub fn hash_maps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let score = scores.get("Blue");

    match score {
        Some(value) => println!("The score of Blue is {}", value),
        None => println!("There is no such key on the hashmap"),
    }

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut creating_new_scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:#?}", creating_new_scores);

    // overwrite
    creating_new_scores.insert(String::from("Blue"), 20);

    // create if not exists
    creating_new_scores.entry(String::from("Red")).or_insert(30);
    // update from previous

    let entry = creating_new_scores
        .entry(String::from("Yellow"))
        .or_insert(40);
    *entry += 1;

    for (key, value) in &creating_new_scores {
        println!("{}: {}", key, value);
    }
}
