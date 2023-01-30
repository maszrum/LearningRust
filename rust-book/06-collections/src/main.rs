use std::collections::HashMap;

fn main() {
    vectors();
    strings();
    hash_maps();
}

fn vectors() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(100);

    let first = &mut v[0];
    *first = 5;

    let first = &v[0];

    let last = v.last().unwrap();

    println!("first: {}, last: {}", first, last);
}

fn strings() {
    let mut s = String::new();
    s.push_str("str");
    println!("s = {}", s);

    let str_from_literal = "string".to_string();
    println!("from literal: {}", str_from_literal);
}

fn hash_maps() {
    let mut scores = HashMap::new();

    // .get()

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let score = scores
        .get("Blue")
        .copied()
        .unwrap_or(0);

    println!("score = {}", score);

    // iterate

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // adding when is not present

    scores
        .entry(String::from("Red"))
        .or_insert(4);

    // update a value based on the old value

    let yellow_score = scores
        .entry(String::from("Yellow"))
        .or_insert(3);

    *yellow_score += 10;

    let blue_score = scores.get_mut("Blue").unwrap();
    *blue_score += 20;
}
