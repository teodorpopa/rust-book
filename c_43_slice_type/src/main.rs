fn main() {
    let s = String::from("Hello World");
    let word = first_word(&s);

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("Word is: {}", word);

    println!("First word: {}, second word: {}", hello, world);

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}