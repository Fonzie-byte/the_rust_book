fn main() {
    let s = String::from("hello"); // String comes into scope.

    // `s` is still available here.

    takes_ownership(s); // String moves into function, out of scope.

    // `s` is no longer available.

    let x = 5; // Integer comes into scope.

    // `x` is still available here.

    makes_copy(x); // Integer has Copy; still in scope!

    // And `x` is also available here.
}

fn takes_ownership(some_string: String) {
    // String moves into scope.
    println!("{some_string}");
} // String goes out of scope and it dropped.

fn makes_copy(some_integer: i32) {
    // Integer copies into scope.
    println!("{some_integer}");
} // Integer "goes out of scope", but nothing happened.
