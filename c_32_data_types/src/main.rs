fn main() {
    //scalar types
    // signed int: -(2^n - 1) to 2^n-1 - 1
    // unsigned int: 0 to 2^n - 1
    // i8, i16, i32, i64, i128, isize
    // u8, u16, u32, u64, u128, usize
    let i1 :i32 = 100;
    let i2 :i64 = 1000;

    // floating-point
    let f1 = 2.0;
    let f2 :f32 = 3.0;

    // numeric operations

    // addition
    let sum = 5 + 10;

    // substraction
    let diff = 05.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // -1

    //remainder
    let remainder = 43 % 5;


    // boolean type
    let t = true;
    let f :bool = false;


    // character type
    let c = 'z';
    let z :char = 'Z';
    let emoji = '😻';


    // compound types

    // tuple
    let tup :(i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let tup2: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup2.0;
    let six_point_four = tup2.1;
    let one = tup2.2;


    // array type
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let a2: [i32; 5] = [1, 2, 3, 4, 5];
    let a2 = [3; 5]; // init with all values the same

    let first = a[0];
    let second = a[1];
}
