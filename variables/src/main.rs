// CONSTANTS
// You declare constants using the const keyword instead of the let keyword, and the type of the value must be annotated.
// Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.
// The last difference is that constants may be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime.

// VARIABLE SHADOWING
// Shadowing is different from marking a variable as mut, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword.
// By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
// The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
    const HOURS_IN_DAY: u32 = 24;
    const MINUTES_IN_HOUR: u32 = 60;
    const SECONDS_IN_MINUTE: u32 = 60;
    println!(
        "There are {} hours in a day, {} minutes in an hour and {} seconds in a minute",
        HOURS_IN_DAY, MINUTES_IN_HOUR, SECONDS_IN_MINUTE
    );
    println!(
        "There are {} seconds in an hour!",
        MINUTES_IN_HOUR * SECONDS_IN_MINUTE
    );
    println!(
        "There are {} seconds in a day!",
        HOURS_IN_DAY * MINUTES_IN_HOUR * SECONDS_IN_MINUTE
    );
    const G: f64 = 9.81;
    println!("Gravitational constant (G) = {}", G);
    let is_ready = true;
    println!("Is it ready? {}", is_ready);
    let month = (1, "January");
    println!("{} -> {}", month.0, month.1);
    let (id, name) = month;
    println!("{} -> {}", id, name);
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    for month in months.iter() {
        println!("{}", month);
    }
}
