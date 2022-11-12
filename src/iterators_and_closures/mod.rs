use std::time::Duration;

pub fn run_iterator() {
    let mut v1 = vec![1,2,3];
    let v1_iter = v1.iter_mut();
    for v in v1_iter {
        *v = 4;
    }
    println!("{:?}", v1);

    let v1 = vec![1,2,3,4];
    let total: i32 = v1.iter().sum();
    println!("{:?} --- sum={}", v1, total);

    let v1 = vec![1,2,3,4];
    let v1: Vec<i32> = v1
        .iter()
        .map(|x| x + 1)
        .collect();
    println!("{:?}", v1);

    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    pub fn shoes_in_size(shoes: &Vec<Shoe>, shoe_size: u32) -> Vec<&Shoe> {
        let shoes: Vec<&Shoe> = shoes.iter().filter(|x| { x.size == shoe_size }).collect();
        shoes
    }

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(&shoes, 10);
    println!("{:?} {:?}", in_my_size, shoes);
}



pub fn run() {
    let store: Inventory = Inventory { shirts: vec![
        ShirtColor::Blue,
        ShirtColor::Red,
        ShirtColor::Red,
        ShirtColor::Blue,
        ShirtColor::Blue,
    ] };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.give_away(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.give_away(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

pub fn run2() {
    let expensive_function = |num: i32| -> i32{
        println!("Calculating ...");
        std::thread::sleep(Duration::from_secs(2));
        num * num
    };

    let value = expensive_function(32);
    println!("{value}");
}

pub fn run_borrowable() {
    let list = vec![1,2,3];
    let only_borrows = || println!("{:?}", list);
    only_borrows();
}

pub fn run_mutable_borrowable() {
    let mut list = vec![1,2,3];
    let mut borrows_and_mut = || {
        println!("{:?}", list);
        list.push(4);
    };
    // println!("{:?}", list); // This cannot work before changing the list below
    borrows_and_mut();
    println!("{:?}", list);
}

pub fn run_movable() {
    let list = vec![1, 2, 3, 4];
    let list = std::thread::spawn(move || {
        let list = list; // moving the list to the new thread
        println!("{:?}", list);
        list
    }).join().unwrap();
    println!("{:?}", list);
}

pub fn run_sort_by_key_example() {

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue
}

pub struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {
    fn give_away(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut number_red = 0;
        let mut number_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => number_red += 1,
                ShirtColor::Blue => number_blue += 1,
            }
        };

        if number_red > number_blue {
            ShirtColor::Red
        }
        else {
            ShirtColor::Blue
        }
    }
}
