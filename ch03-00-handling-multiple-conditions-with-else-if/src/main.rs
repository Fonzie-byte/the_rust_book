fn main() {
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("foobar")
        } else if n % 5 == 0 {
            println!("bar")
        } else if n % 3 == 0 {
            println!("foo")
        } else {
            println!("{n}")
        }
    }
}
