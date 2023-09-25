use rand::Rng;

fn main() {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
    let consonants = [
        'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w',
        'x', 'z',
    ];
    let mut currently_vowel = false;

    let mut generated_string = String::new();

    for _ in 0..8 {
        // You can use an if-statement even in variable binding.
        let next_character = if currently_vowel {
            vowels[rand::thread_rng().gen_range(0..6)]
        } else {
            consonants[rand::thread_rng().gen_range(0..20)]
        };

        generated_string.push(next_character);
        currently_vowel = !currently_vowel;
    }

    println!("{generated_string}");
}
