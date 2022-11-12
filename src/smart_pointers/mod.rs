mod con_list_example;
mod my_box_example;

pub fn run() {
    // run_simple();
    // con_list_example::run();
    my_box_example::run();
}

fn run_simple() {
    let x = Box::new(5);
    println!("b = {}", x);
}