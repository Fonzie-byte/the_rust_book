const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut num = 5;
    println!("The number is {num}");
    num = 6;
    println!("The number is {num}");

    // consts are available even from the global scope, but can never be `mut`
    println!("Three hours is {THREE_HOURS_IN_SECONDS} seconds");

    // Shadowing temprarily overrides a variable
    let points = 5;
    let points = points + 1;
    {
        let points = points * 2;
        println!("Inner scope scored {points} points!");
    }
    println!("Outer scope is left with {points} points...");

    // This can be used to "change a variable's type"
    let spaces = "    ";
    let spaces = spaces.len();

    println!("Indented with {spaces} spaces");
}
