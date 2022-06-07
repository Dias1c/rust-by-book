// Constants
// constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    variables();
    shadowing();
}

fn variables() {
    // Variables
    // let x = 5; // Will not work, cause its immutable;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn shadowing() {
    // Shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    // Also Shadowing
    let spaces = "   ";
    println!("The value of spaces is: '{}'", spaces);
    
    let spaces = spaces.len();
    println!("The value of spaces is: '{}'", spaces);
}