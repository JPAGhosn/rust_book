use std::collections::HashMap;

pub fn run() {
    let mut v = vec![1, 2, 3];
    v.push(5);
    let third = v.get(2).expect("Out of bound");
    println!("{third}");

    /// cannot push a new element after borrowing it
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);

    /// For in a collection
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    /// For loop and change the values directly
    let mut v = vec![100, 32, 57];
    for i in &mut v { // mut
        *i += 50;
    }

    /// When the vector goes out of scope, all its elements are dropped
    {
        let v = vec![1, 2, 3, 4];
    } // <- v goes out of scope and is freed here

    /// Hashmaps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    /// get values from Hashmaps
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).expect("Element not found!");
    println!("Blue team score: {score}");

    /// Iterating a hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    println!("{:?}", scores);

    /// Adding if not present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    /// Updating a value based on the old one
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

}