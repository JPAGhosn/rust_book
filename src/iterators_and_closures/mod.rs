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
