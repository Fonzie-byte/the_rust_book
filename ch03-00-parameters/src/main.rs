fn print_number(number: i32) {
    println!("The number you passed is {number}.");
}

fn print_labeled_measurement(amount: i32, unit_label: char) {
	println!("The measurement is {amount}{unit_label}.");
}

fn main() {
    print_number(16);

    print_labeled_measurement(6, 'm');
}
