pub mod models;
use models::{ ip_addr_kind::IpAddrKind, ip_addr::IpAddr};
use crate::enums_and_pattern_matching_6::models::message::Message;

pub fn run() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let quit = Message::Quit;

    if let Message::Quit = quit {
        println!("Quiting the application");
    } else {
        println!("Not quiting the application");
    }

    let write = Message::Write("Hello there".into());
    quit.call();
    write.call();
}

fn route(ip_kind: IpAddrKind) {}

