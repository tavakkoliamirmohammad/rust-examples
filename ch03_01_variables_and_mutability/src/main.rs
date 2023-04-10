fn main() {
    // variable definition
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // constant definition
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // shadowing
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // shadowing (change of type)
    let spaces = "   ";
    let spaces = spaces.len();

    // shadowing vs mut
    let mut spaces = "   ";
    // spaces = spaces.len(); <- Compile error expect &String not &u32
}
