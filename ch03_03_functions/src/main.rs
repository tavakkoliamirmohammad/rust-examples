fn main() {
    println!("Hello, world!");
    another_function();
    another_function_with_params(5);
    print_labeled_measurement(5, 'h');

    // Scope block is an expression
    let y = {
        let x = 3;
        x + 1 // Note we are not using semicolon
    };

    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_params(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
