pub fn run() {
    // mutability
    let mut x = 5;
    x = 6;

    // shadowing
    let mut x = "7";

    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // convert
    let guess: u32 = "42".parse().expect("Not a number!");

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup.0;
    let (x, y, z) = tup;

    // Array
    let a = [1, 2, 3, 4, 5];
    let first = a[0];

    // functions
    fn plus_one(x: i32) -> i32 {
        x + 1
    }

    // statements
    let y = {
        let x = 3;
        x + 1
    };

    // if else
    let number = 3;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // let if
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    // loop with labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    // for loop
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }

}