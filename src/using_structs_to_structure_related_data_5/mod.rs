pub fn run() {

    /// Struct
    let email = "someone@example.com".into();
    let mut user1 = User {
        email,
        username: dbg!(String::from("someusername123")),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    /// Parametless struct
    let black = Color(0, 0, 0);

    /// Unit like structs
    let subject = AlwaysEqual;

    /// Methods with structs
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", rect1.area()); // automatically deref

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    // Fatory
    let s = Rectangle::square(30);
}

/// Struct
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/// Methods of struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// factories
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/// Parametless structs
struct Color(i32, i32, i32);

/// Unit like struct
struct AlwaysEqual;
