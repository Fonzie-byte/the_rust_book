fn main() {
    // May note base before (0x hex, 0b binary...)
    // May note data type after (u32 unsigned-32-bits)
    // May use `_` to separate
    let colour = 0xf7_4b_00u32;

    let x = 2f32;
    let y: f32 = 3.4;

    println!("Ferris is coloured {colour:?}");
    println!("And located at {x:?} x {y:?}");

    // ' makes a simple, primitive char
    let twitter = '𝕏';
    println!("Twitter is {twitter} and is a char");

    // " is a string, a more complex compound
    let twitter = "𝕏";
    println!(
        "Twitter is {twitter} and is {} characters long",
        twitter.len()
    );
}
