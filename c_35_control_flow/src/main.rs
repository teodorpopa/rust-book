fn main() {
    let num = 4;

    // simple if/else
    if num < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // ====================================

    let check = 3;

    // check if number exists
    if check != 0 {
        println!("number is three");
    }

    // ====================================

    let div = 6;

    // simple if/else if/else
    if div % 4 == 0 {
        println!("Number is divisible by 4");
    } else if div % 3 == 0 {
        println!("Number is divisible by 3");
    } else if div % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    // ====================================

    let condition = true;
    // set statement based on condition
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // ====================================

    let mut counter = 0;
    // create a loop and assign the result to result variable
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // ====================================

    let mut count = 0;
    // named loops
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // ====================================

    let mut number = 3;
    // while loop
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // ====================================

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    // loop array
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // ====================================

    let a = [10, 20, 30, 40, 50];
    // foreach element in array
    for element in a {
        println!("the value is: {element}");
    }

    // ====================================

    // loop reverse range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
