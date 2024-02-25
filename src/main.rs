// // // use std::io;
// // // fn main() {
// // //     // Mutablility
// // //     // let mut x = 5;
// // //     // println!("The value of x is: {x}");
// // //     // x = 6;
// // //     // println!("The value of x is: {x}");

// // //     // constants

// // //     // const VARIABLE:i32 = 90;
// // //     // println!("{}" , VARIABLE);

// // //     // shadowing

// // //     // let x = 5;
// // //     // let x = x + 1;

// // //     // {
// // //     //     let x = x * 2;
// // //     //     println!("Inside the scope {x}");
// // //     // }

// // //     // println!("Outside the scope {x}");

// // //     // Data types

// // //     //     Length	Signed	Unsigned
// // //     //       8-bit	i8	    u8
// // //     //      16-bit	i16	    u16
// // //     //      32-bit	i32	    u32
// // //     //      64-bit	i64	    u64
// // //     //      128-bit	i128    u128
// // //     //      arch    isize	usize

// // //     // floating point values
// // //     // let x = 90.90;
// // //     // println!("{}", x);

// // //     // Numneric operations
// // //     //  // addition
// // //     //  let sum = 5 + 10;

// // //     //  // subtraction
// // //     //  let difference = 95.5 - 4.3;

// // //     //  // multiplication
// // //     //  let product = 4 * 30;

// // //     //  // division
// // //     // let _quotient = 56.7 / 32.2;
// // //     // let truncated = -6 / 3; // Results in -1
// // //     // println!("{}", truncated);
// // //     //  // remainder
// // //     //  let remainder = 43 % 5;

// // //     // Boolean types
// // //     // let ok = false;
// // //     // println!("{}", ok);

// // //     // character types
// // //     // let c = 'z';
// // //     // let z: char = '*'; // with explicit type annotation
// // //     // let heart_eyed_cat = '😻';

// // //     // tuple
// // //     // let tup = (500, 6.4, 1);

// // //     // let (x, y, z) = tup;

// // //     // println!("The value of y is: {y}");

// // //     // let x: (i32, f64, u8) = (500, 6.4, 1);

// // //     // let five_hundred = x.0;

// // //     // let six_point_four = x.1;

// // //     // let one = x.2;

// // //     // Arrays in rust
// // //     // let a = [1, 2, 3, 4, 5];
// // //     // println!("Please enter an array index.");

// // //     // let mut index = String::new();

// // //     // io::stdin()
// // //     //     .read_line(&mut index)
// // //     //     .expect("Failed to read line");

// // //     // let index: usize = index
// // //     //     .trim()
// // //     //     .parse()
// // //     //     .expect("Index entered was not a number");

// // //     // let element = a[index];

// // //     // println!("The value of the element at index {index} is: {element}");

// // //     // FUNCTIONS
// // //     // println!("Hello world");

// // //     another_function(5, "ayush");

// // //     // Control Flow
// // //     conditional_while();
// // //     if_expression();
// // //     loops();
// // //     returning_values_from_loop();
// // //     multipleLoops();
// // //     for_loops_method();
// // //     celcius_to_fahrenheit();
// // //     fibonacci();
// // // }

// // // fn another_function(x: i32, y: &str) {
// // //     let ans = five();
// // //     println!("Another functino , {x} , {ans} , {y}");
// // // }

// // // fn five() -> i32 {
// // //     return 5;
// // // }

// // // fn if_expression() {
// // //     // let number = 3;
// // //     // if number % 4 == 0 {
// // //     //     println!("number is divisible by 4");
// // //     // } else if number % 3 == 0 {
// // //     //     println!("number is divisible by 3");
// // //     // } else if number % 2 == 0 {
// // //     //     println!("number is divisible by 2");
// // //     // } else {
// // //     //     println!("number is not divisible by 4, 3, or 2");
// // //     // }

// // //     let condition = true;
// // //     let number = if condition { 5 } else { 6 };

// // //     println!("The value of number is: {number}");
// // // }

// // // fn loops() {
// // //     loop {
// // //         println!("again!");
// // //         break;
// // //     }
// // // }

// // // fn returning_values_from_loop() {
// // //     let mut counter = 0;

// // //     let result = loop {
// // //         counter += 1;

// // //         if counter == 10 {
// // //             break counter * 2;
// // //         }
// // //     };

// // //     println!("The result is {result}");
// // // }

// // // fn multipleLoops() {
// // //     let mut count = 0;
// // //     'counting_up: loop {
// // //         println!("count = {count}");
// // //         let mut remaining = 10;

// // //         loop {
// // //             println!("remaining = {remaining}");
// // //             if remaining == 9 {
// // //                 break;
// // //             }
// // //             if count == 2 {
// // //                 break 'counting_up;
// // //             }
// // //             remaining -= 1;
// // //         }

// // //         count += 1;
// // //     }
// // //     println!("End count = {count}");
// // // }

// // // fn conditional_while() {
// // //     let mut number = 3;

// // //     while number != 0 {
// // //         println!("{number}!");

// // //         number -= 1;
// // //     }

// // //     println!("LIFTOFF!!!");
// // // }

// // // fn for_loops() {
// // //     let a = [10, 20, 30, 40, 50];

// // //     for element in a {
// // //         println!("the value is: {element}");
// // //     }
// // // }

// // // fn for_loops_method() {
// // //     for number in (0..4) {
// // //         println!("{number}!");
// // //     }
// // //     println!("LIFTOFF!!!");
// // // }

// // // fn convert_fahrenheit_to_celsius() {
// // //     let mut fahrenheit = String::new();
// // //     io::stdin()
// // //         .read_line(&mut fahrenheit)
// // //         .expect("failed to read line");

// // //     let fahrenheit: i32 = fahrenheit
// // //         .trim()
// // //         .parse()
// // //         .expect("Please enter a valid integer");

// // //     println!("{}", fahrenheit);

// // //     let celcius = (fahrenheit - 32) * 5 / 9;
// // //     println!("celcius is {celcius}");
// // // }

// // // fn celcius_to_fahrenheit() {
// // //     let mut celcisus = String::new();
// // //     io::stdin()
// // //         .read_line(&mut celcisus)
// // //         .expect("failed to read line");

// // //     let celcisus: i32 = celcisus
// // //         .trim()
// // //         .parse()
// // //         .expect("Please enter a valid integer");

// // //     println!("{}", celcisus);

// // //     let fahrenheit = (celcisus - 32) * 5 / 9;
// // //     println!("fahrenheit is {fahrenheit}");
// // // }

// // // fn fibonacci() {
// // //     let mut n = String::new();
// // //     io::stdin().read_line(&mut n).expect("Error input");

// // //     let n: usize = n.trim().parse().expect("Please enter a number");
// // //     let mut dp = vec![-1; n + 1];
// // //     dp[0] = 0;
// // //     dp[1] = 1;

// // //     for i in 2..=n {
// // //         dp[i] = dp[i - 1] + dp[i - 2];
// // //     }

// // //     println!("The fibonacci sequence at index {n} is {}", dp[n]);
// // // }

// // // fn main() {
// // //     string_heap();
// // //     ownership_function();
// // // }

// // // fn string_heap() {
// // //     // let s1 = String::from("hello");
// // //     // let s2 = s1;
// // //     let s1 = String::from("hello");
// // //     let s2 = s1.clone();

// // //     println!("s1 = {}, s2 = {}", s1, s2);
// // //     println!("{}, world!", s2);
// // // }

// // // fn ownership_function() {
// // //     let s = String::from("hello"); // s comes into scope

// // //     takes_ownership(s); // s's value moves into the function...
// // //                         // ... and so is no longer valid here

// // //     let x = 5; // x comes into scope

// // //     makes_copy(x); // x would move into the function,
// // //                    // but i32 is Copy, so it's okay to still
// // //                    // use x afterward
// // // }

// // // fn takes_ownership(some_string: String) {
// // //     // some_string comes into scope
// // //     println!("{}", some_string);
// // // } // Here, some_string goes out of scope and `drop` is called. The backing
// // //   // memory is freed.

// // // fn makes_copy(some_integer: i32) {
// // //     // some_integer comes into scope
// // //     println!("{}", some_integer);
// // // } // Here, some_integer goes out of scope. Nothing special happens.

// // // fn return_values_and_scope() {
// // //     let s1 = gives_ownership(); // gives_ownership moves its return
// // //                                 // value into s1

// // //     let s2 = String::from("hello"); // s2 comes into scope

// // //     let s3 = takes_and_gives_back(s2); // s2 is moved into
// // //                                        // takes_and_gives_back, which also
// // //                                        // moves its return value into s3
// // // }

// // // fn gives_ownership() -> String {
// // //     // gives_ownership will move its
// // //     // return value into the function
// // //     // that calls it

// // //     let some_string = String::from("yours"); // some_string comes into scope

// // //     some_string // some_string is returned and
// // //                 // moves out to the calling
// // //                 // function
// // // }

// // // // This function takes a String and returns one
// // // fn takes_and_gives_back(a_string: String) -> String {
// // //     // a_string comes into
// // //     // scope

// // //     a_string // a_string is returned and moves out to the calling function
// // // }

// // // reference and borrowing

// // // fn main() {

// // // }

// // // fn reference_and_borrowing() {
// // //     let s1 = String::from("hello");

// // //     let len = calculate_length(&s1);

// // //     println!("The length of '{}' is {}.", s1, len);
// // // }

// // // fn calculate_length(s: &String) -> usize {
// // //     s.len()
// // // }

// // // fn change(some_string: &mut String) {
// // //     some_string.push_str(", world");
// // // }

// // // fn dangeling_references(){
// // //     let reference_to_nothing = dangle();
// // // }

// // // fn dangle() -> &String {
// // //     let s = String::from("hello");

// // //     s
// // // }

// // // The slice type
// // fn main() {}

// // fn first_word(s: &String) -> usize {
// //     let bytes = s.as_bytes();

// //     for (i, &item) in bytes.iter().enumerate() {
// //         if item == b' ' {
// //             return i;
// //         }
// //     }

// //     s.len()
// // }

// // fn string_slices() {
// //     let s = String::from("hello world");

// //     let hello = &s[0..5];
// //     let world = &s[6..11];
// // }

// // fn first_word_with_slice(s: &String) -> &str {
// //     let bytes = s.as_bytes();

// //     for (i, &item) in bytes.iter().enumerate() {
// //         if item == b' ' {
// //             return &s[0..i];
// //         }
// //     }

// //     &s[..]
// // }

// // // fn string_literal_slice() {
// // //     let my_string = String::from("hello world");

// // //     // `first_word` works on slices of `String`s, whether partial or whole
// // //     let word = first_word(&my_string[0..6]);
// // //     let word = first_word(&my_string[..]);
// // //     // `first_word` also works on references to `String`s, which are equivalent
// // //     // to whole slices of `String`s
// // //     let word = first_word(&my_string);

// // //     let my_string_literal = "hello world";

// // //     // `first_word` works on slices of string literals, whether partial or whole
// // //     let word = first_word(&my_string_literal[0..6]);
// // //     let word = first_word(&my_string_literal[..]);

// // //     // Because string literals *are* string slices already,
// // //     // this works too, without the slice syntax!
// // //     let word = first_word(my_string_literal);
// // // }

// // struct in Rust

// // struct User {
// //     active: bool,
// //     username: String,
// //     email: String,
// //     sign_in_count: u64,
// // }

// // struct Color(i32, i32, i32);
// // struct Point(i32, i32, i32);

// // struct AlwaysEqual;

// // #[derive(Debug)]
// // struct Rectangle {
// //     width: u32,
// //     height: u32,
// // }

// // fn main() {
// //     let user1 = User {
// //         active: true,
// //         username: String::from("someusername123"),
// //         email: String::from("someone@example.com"),
// //         sign_in_count: 1,
// //     };

// //     let user2 = User {
// //         email: String::from("another@example.com"),
// //         ..user1
// //     };

// //     let black = Color(0, 0, 0);
// //     let origin = Point(0, 0, 0);

// //     let subject = AlwaysEqual;

// //     let rect1 = Rectangle {
// //         width: 30,
// //         height: 50,
// //     };

// //     println!("rect1 is {:?}", rect1);
// // }

// // fn build_user(email: String, username: String) -> User {
// //     User {
// //         active: true,
// //         username: username,
// //         email: email,
// //         sign_in_count: 1,
// //     }
// // }

// // Method syntax
// // #[derive(Debug)]
// // struct Rectangle {
// //     width: u32,
// //     height: u32,
// // }

// // impl Rectangle {
// //     fn area(&self) -> u32 {
// //         self.width * self.height
// //     }
// //     fn can_hold(&self, other: &Rectangle) -> bool {
// //         self.width > other.width && self.height > other.height
// //     }
// //     fn square(size: u32) -> Self {
// //         Self {
// //             width: size,
// //             height: size,
// //         }
// //     }
// // }

// // fn main() {
// //     let rect1 = Rectangle {
// //         width: 30,
// //         height: 50,
// //     };

// //     println!(
// //         "The area of the rectangle is {} square pixels.",
// //         rect1.area()
// //     );
// //     let rect2 = Rectangle {
// //         width: 10,
// //         height: 40,
// //     };
// //     let rect3 = Rectangle {
// //         width: 60,
// //         height: 45,
// //     };

// //     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
// //     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
// // }

// // enums and pattern matching
// enum IpAddrKind {
//     V4,
//     V6,
// }
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// enum IpAddr2 {
//     V4(String),
//     V6(String),
// }

// struct Ipv4Addr {
//     // --snip--
// }

// struct Ipv6Addr {
//     // --snip--
// }

// enum IpAddr3 {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {
//         // method body would be defined here
//     }
// }

// enum Option<T> {
//     None,
//     Some(T),
// }

// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//     route(IpAddrKind::V4);
//     route(IpAddrKind::V6);

//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };

//     let home = IpAddr2::V4(String::from("127.0.0.1"));

//     let loopback = IpAddr2::V6(String::from("::1"));

//     let some_number = Some(5);
//     let some_char = Some('e');

//     // let absent_number: Option<i32> = None;
// }

// fn route(ip_kind: IpAddrKind) {}

// The match control flow construct

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// #[derive(Debug)] // so we can inspect the state in a minute
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }

// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         other => move_player(other),
//     }
// }

// fn add_fancy_hat() {}
// fn remove_fancy_hat() {}
// fn move_player(num_spaces: u8) {}

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// Concise Control Flow with if let

// fn main() {
//     let config_max = Some(3u8);
//     if let Some(max) = config_max {
//         println!("The maximum is configured to be {}", max);
//     }

//     let mut count = 0;
//     if let Coin::Quarter(state) = coin {
//         println!("State quarter from {:?}!", state);
//     } else {
//         count += 1;
//     }
// };
// vector
//     enum SpreadsheetCell {
//         Int(i32),
//         Float(f64),
//         Text(String),
//     }
//
//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Text(String::from("blue")),
//         SpreadsheetCell::Float(10.12),
//     ];

// v.push(9);
//}

// fn main() {
//     let mut s = String::new();
//     let data = "initial contents";
//     let s = data.to_string();
//     // the method also works on a literal directly:
//     let s = "initial contents".to_string();
// }

//     let mut scores = HashMap::new();
//     scores.insert(String::from("blue"), 0);
//     scores.insert(String::from("yellow"), 0);
//
//     let team_name = String::from("blue");
//     let score = scores.get(&team_name).copied().unwrap(0);
//     for (key, value) in &scores {
//         println!("{key}: {value}");
//     }
//     scores.entry(String::from("Yellow")).or_insert(50);
//     scores.entry(String::from("Blue")).or_insert(50);
// }

// error handeling

// use std::io::{self, Read};
//
// fn main() {
//     let greeting_file_result = File::open("hello.txt");
//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => panic!("Problems opening the file: {:?}", error),
//     };
//     // //     // let greeting_file_2 = match error.kind(){
//     // //     //     ErrorKind::NotFound => match File::create("hello.txt") {
//     // //     //         Ok(fx) => fx,
//     // //     //         Err(e) => panic!("Proble creating the file {}" , e);
//     // //     //     },
//     // //     //     other_error => {
//     // //     //         panic!("Proble opeing the file {}" , other_error);
//     // //     //     }
//     // };
// }
//
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//     let mut username = String::new();
//
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }
//
// generics
// fn main() {
//
// }
//
// fn duplications() {
//     let number_list = vec![43, 34, 43, 32333, 32334234];
//
//     let mut largest = &number_list[0];
//
//     for number in &number_list {
//         if number > largest {
//             largest = number;
//         }
//     }
//     println!("The largest number is {}", largest);
//
//
//     let number_list = vec![3434,4,34,3,43,4,34,34,3,43,4,234,234,234,90];
//
//     let result = largest(&number_list);
//     println("The largest number is {}" , result);
// }
//
// fn largest(list:&[i32]) -> &i32 {
//     let mut largest = &list[0];
//
//     for item in list{
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
//
// }
//

// generic data type

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
