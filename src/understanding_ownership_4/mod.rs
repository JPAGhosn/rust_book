pub fn run() {

    { // s is not valid here, it’s not yet declared
        let s: String = "hello".into(); // s is valid from this point forward
    } // this scope is now over, and s is no longer valid


    let s1 = String::from("hello");
    let s2 = s1; // s1 is invalid now
    // println!("{s1}"); // won't work
    let mut s3 = s2.clone(); // works

    let len = calculate_length(&s3);
    println!("{len}");

    change(&mut s3);
    println!("{s3}");


    /// ========= Borrowing rules ===========

    /// Rule 1:Cannot borrow more than a mut reference if it was borrowed before unless the first one goes out of scope
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;

    /// Rule 2: You can prevent rule 1 by using the variable then borrowing it again
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    let r3 = &mut s; // no problem
    println!("{}", r3);

    /// ========= Slice type ===========
    /// Reference part of a collection or array
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11]; // d at index 10
    let world2 = &s[6..11]; // d at index 10

    let mut s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); // error!

}

/// Taking ownership
fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

/// Giving ownership
// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope
    a_string  // a_string is returned and moves out to the calling function
}

/// Borrowing value
fn calculate_length(s: &String) -> usize {
    s.len()
}

/// Borrowing and change its content directly
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Return reference because we want to tell the compiler to prevent s from being modified
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
