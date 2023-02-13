fn main() {
    println!("Hello, world!");

    another_function();

    another_function_w_params(5);

    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");


    let po = plus_one(5);
    println!("The value of x is: {po}");
}

// simple function
fn another_function() {
    println!("Another function.");
}

// function with parameter
fn another_function_w_params(x: i32) {
    println!("The value of x is: {x}");
}

// function with multiple parameters
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// function with unnamed return
fn five() -> i32 {
    5
}

// function with expression return
fn plus_one(x: i32) -> i32 {
    x + 1
}