use std::collections::HashMap;

// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
pub fn text_interface() {}

// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
pub fn pig_latin() {
    let my_string = String::from("first apple");
    let mut pig_string = String::new();
    for word in my_string.split(" ") {
        for (index, char) in word.chars().enumerate() {
            if index == 0 {
                if char == 'a' || char == 'e' || char == 'i' || char == 'o' || char == 'u' {
                    pig_string.push_str(&format!("{}-hay", word));
                } else {
                    pig_string.push_str(&format!("{}-{}ay", &word[1..], &word[0..1]));
                }
                pig_string.push_str(" ");
                break;
            }
        }
    }
    println!("{}", pig_string);
}

// Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.
pub fn soln1() {
    let mut list = vec![1, 2, 3, 3, 3, 4, 5];
    list.sort();
    println!("Median is {}", list[list.len() / 2]);
    let mut sum = 0;
    let mut hs = HashMap::new();
    for i in list.iter() {
        sum += i;
        let entry = hs.entry(i).or_insert(0);
        *entry += 1;
    }
    println!("Mean is {}", sum / list.len());
    let mut rpt = 0;
    let mut mode = 0;
    for (k, v) in hs {
        if v > rpt {
            mode = *k;
            rpt = v;
        }
    }
    println!("Mode is {}", mode);
}

// FizzBuzz
pub fn fizz_buzz(from: i32, to: i32) {
    for i in from..to {
        match (i % 3, i % 5) {
            (0, 0) => println!("fizzbuzz"),
            (0, _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            (_, _) => println!("{}", i),
        }
    }
}
