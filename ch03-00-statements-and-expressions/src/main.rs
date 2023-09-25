fn main() {
    let y = {
        // Statement, does not result in anything.
        let x = 3;
        // Expression (no ;), results in a value, so this block results in 4.
        x + 1
    };

    println!("The value of y is {y}");
}
