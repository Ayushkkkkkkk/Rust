use std::io;
fn main() {
    // Mutablility
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // constants

    // const VARIABLE:i32 = 90;
    // println!("{}" , VARIABLE);

    // shadowing

    // let x = 5;
    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("Inside the scope {x}");
    // }

    // println!("Outside the scope {x}");

    // Data types

    //     Length	Signed	Unsigned
    //       8-bit	i8	    u8
    //      16-bit	i16	    u16
    //      32-bit	i32	    u32
    //      64-bit	i64	    u64
    //      128-bit	i128    u128
    //      arch    isize	usize

    // floating point values
    // let x = 90.90;
    // println!("{}", x);

    // Numneric operations
    //  // addition
    //  let sum = 5 + 10;

    //  // subtraction
    //  let difference = 95.5 - 4.3;

    //  // multiplication
    //  let product = 4 * 30;

    //  // division
    // let _quotient = 56.7 / 32.2;
    // let truncated = -6 / 3; // Results in -1
    // println!("{}", truncated);
    //  // remainder
    //  let remainder = 43 % 5;

    // Boolean types
    // let ok = false;
    // println!("{}", ok);

    // character types
    // let c = 'z';
    // let z: char = '*'; // with explicit type annotation
    // let heart_eyed_cat = '😻';

    // tuple
    // let tup = (500, 6.4, 1);

    // let (x, y, z) = tup;

    // println!("The value of y is: {y}");

    // let x: (i32, f64, u8) = (500, 6.4, 1);

    // let five_hundred = x.0;

    // let six_point_four = x.1;

    // let one = x.2;

    // Arrays in rust
    // let a = [1, 2, 3, 4, 5];
    // println!("Please enter an array index.");

    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");

    // let element = a[index];

    // println!("The value of the element at index {index} is: {element}");

    // FUNCTIONS
    // println!("Hello world");

    another_function(5, "ayush");

    // Control Flow
    conditional_while();
    if_expression();
    loops();
    returning_values_from_loop();
    multipleLoops();
    for_loops_method();
    celcius_to_fahrenheit();
    fibonacci();
}

fn another_function(x: i32, y: &str) {
    let ans = five();
    println!("Another functino , {x} , {ans} , {y}");
}

fn five() -> i32 {
    return 5;
}

fn if_expression() {
    // let number = 3;
    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn loops() {
    loop {
        println!("again!");
        break;
    }
}

fn returning_values_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn multipleLoops() {
    let mut count = 0;
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
}

fn conditional_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loops() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn for_loops_method() {
    for number in (0..4) {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn convert_fahrenheit_to_celsius() {
    let mut fahrenheit = String::new();
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("failed to read line");

    let fahrenheit: i32 = fahrenheit
        .trim()
        .parse()
        .expect("Please enter a valid integer");

    println!("{}", fahrenheit);

    let celcius = (fahrenheit - 32) * 5 / 9;
    println!("celcius is {celcius}");
}

fn celcius_to_fahrenheit() {
    let mut celcisus = String::new();
    io::stdin()
        .read_line(&mut celcisus)
        .expect("failed to read line");

    let celcisus: i32 = celcisus
        .trim()
        .parse()
        .expect("Please enter a valid integer");

    println!("{}", celcisus);

    let fahrenheit = (celcisus - 32) * 5 / 9;
    println!("fahrenheit is {fahrenheit}");
}

fn fibonacci() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error input");

    let n: usize = n.trim().parse().expect("Please enter a number");
    let mut dp = vec![-1; n + 1];
    dp[0] = 0;
    dp[1] = 1;

    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    println!("The fibonacci sequence at index {n} is {}", dp[n]);
}
