fn main() {
    let s1 = String::from("Ho, there..!");
    let s2 = s1;

    // This does not work, s1 has been moved to s2.
    //println!("{s1}");

    // This works, s2 is the one pointing to "hello" in the heap, now!
    println!("{s2}");

    // We can clone it, to also copy the heap.
    // Generally, it's best to avoid this.
    let s3 = s2.clone();

    println!("s2 is {s2}, s3 is {s3}");

    // Integers have a fixed size, they're on the stack, they always copy.
    let n1 = 14;
    let n2 = n1;

    /*
     * The same goes for bools, floats, chars and tuples containing only these
     * fixed-size types.
     */

    println!("n1 is {n1}, n2 is {n2}");
}
