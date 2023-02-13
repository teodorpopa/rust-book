fn main() {
    // The stack stores values in the order it gets them and removes the values in the opposite order.
    // This is referred to as last in, first out. Think of a stack of plates: when you add more plates,
    // you put them on top of the pile, and when you need a plate, you take one off the top. Adding or
    // removing plates from the middle or bottom wouldn’t work as well! Adding data is called pushing
    // onto the stack, and removing data is called popping off the stack. All data stored on the stack
    // must have a known, fixed size. Data with an unknown size at compile time or a size that might
    // change must be stored on the heap instead.

    // The heap is less organized: when you put data on the heap, you request a certain amount of
    // space. The memory allocator finds an empty spot in the heap that is big enough, marks it as
    // being in use, and returns a pointer, which is the address of that location. This process
    // is called allocating on the heap and is sometimes abbreviated as just allocating (pushing
    // values onto the stack is not considered allocating). Because the pointer to the heap is a
    // known, fixed size, you can store the pointer on the stack, but when you want the actual data,
    // you must follow the pointer.

    // Each value in Rust has an owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.

    {
        // s is not valid here, it’s not yet declared
        let s = "Hello"; // s is valid from this point forward
    }    // this scope is now over, and s is no longer valid

    let s = String::from("hello");


    // Bind the value 5 to x; then make a copy of the value in x and bind it to y.
    // We now have two variables, x and y, and both equal 5
    let x = 5;
    let y = x;


    let s1 = String::from("hello");
    let s2 = s1;
    // To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid.


    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // We don’t have a call to clone, but x is still valid and wasn’t moved into y



    // Here are some of the types that implement Copy:
    //
    // All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating-point types, such as f64.
    // The character type, char.
    // Tuples, if they only contain types that also implement Copy. For example,
    // (i32, i32) implements Copy, but (i32, String) does not.


    let s = String::from("hello");      // s comes into scope

    takes_ownership(s);      // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5;                     // x comes into scope

    makes_copy(x);          // x would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x afterward


    // Rust does let us return multiple values using a tuple
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

}
// Here, x goes out of scope, then s. But because s's value was moved, nothing
// special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}