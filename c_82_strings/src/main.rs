fn main() {

    // initialize string

    let mut s = String::new();

    let data = "Initial content";
    let s = data.to_string();

    let s = "Initial content".to_string();

    let s = String::from("Initial content");

    
    // concatenate strings
    
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is: {s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    let s = format!("{s3}-{s2}");

    // slicing

    let hello = "hello";

    let s = &hello[0..4];
    println!("{}", s);


    // iterating
    
    for c in "Зд".chars() {
        println!("{c}");
    }
}

