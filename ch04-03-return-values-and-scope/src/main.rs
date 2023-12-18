fn main() {
    let s1 = gives_ownership(); // Function moves its value to this scope
    println!("I have received what is {s1}");

    let s2 = String::from("hello"); // String comes into scope
    println!("I just came into scope to say {s2}");

    let s3 = takes_and_gives_back(s2); // String moved out of scope and returned

    println!("That's all, folks, {s3}!");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours"); // String comes into scope

    println!("Wait a minute, I'll be {some_string}");

    some_string // String is returned into previous scope
}

fn takes_and_gives_back(mut a_string: String) -> String {
    println!("{a_string}? Did you drop something?");

    a_string = String::from("goodbye");

    a_string
}
