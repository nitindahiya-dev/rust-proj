// fn main(){
//     let x: i8= 8;
//     let y = String::from("Heyy..!!");
//     print!("x:{}",x);
//     print!("y:{}",y);
// }

// ------------------------------------------------------------------------------------

// fn main() {
//     let greeting = String::from("hello rust..");
//     println!("{}", greeting);
//     let xyz: Option<char> = greeting.chars().nth(2);

//     match xyz {
//         Some(c) => println!("{}", c),
//         None => println!("No charcter found"),
//     }

// }

// -----------------------------------------------------------------------------------

// fn main () {
//     let is_even = false;
//     if is_even {
//         println!("yes it is even number");
//     }
//     if !is_even {
//         println!("no it is not a even number")
//     }
// }

// -----------------------------------------------------------------------------------

// fn main(){
//     for i in 0..100 {
//         println!("{}", i)
//     }
// }

// ----------------------------------------------------------------------------------

// fn main(){
//     let sentence = String::from("Hi, i am a sentence");
//     let first_word = get_first_word(sentence);
//     println!("The first word is: {}", first_word);
// }

// fn get_first_word(sentence: String) -> String {
//     let mut ans = String::from("");
//     for char in sentence.chars(){
//         if char == ' ' {
//             break;
//         }
//         ans.push_str(char.to_string().as_str());
//     }
//     return  ans;
// }

// -------------------------------------------------------------------------------------

// fn main() {
//     let n = 1000;
//     for i in 0..n+1 {
//         println!("{}",i);

//     }
// }

// -------------------------------------------------------------------------------------

// fn main() {
//     let a = 10;
//     let b = 20;
//     let sum = sum_of_two_num(a, b);
//     print!("sum of {} and {} numbers is: {}", a, b, sum);
// }

// fn sum_of_two_num(a: i32, b: i32) -> i32 {
//     a + b
// }

// -------------------------------------------------------------------------------------

// fn main() {
//     run_loop();
// }

// fn run_loop() {
//     let mut x = Vec::new();
//     for i in 0..100 {
//         x.push(i);
//     }
//     return  println!("{:?}",x) ;
// }

// -------------------------------------------------------------------------------------

// Memory Management

// fn main() {
//     stack_fn();   // Call the function that uses stack memory
//     heap_fn();    // Call the function that uses heap memory
//     update_string();  // Call the function that changes size of variable at runtime
// }

// fn stack_fn() {
//     // Declare a few integers on the stack
//     let a: i32 = 10;
//     let b: i32 = 20;
//     let c: i32 = a + b;
//     println!("Stack function: The sum of {} and {} is {}", a, b, c);
// }

// fn heap_fn() {
//     // Create a string, which is allocated on the heap
//     let s1: String = String::from("Hello");
//     let s2: String = String::from("World");
//     let combined: String = format!("{} {}", s1, s2);
//     println!("Heap function: Combined string is '{}'", combined);
// }

// fn update_string() {
//     // Start with a base string on the heap
//     let mut s: String = String::from("Initial string");
//     println!("Before update: {}", s);
//     println!("Capacity:{}, Length: {}, pointer: {:p}", s.capacity(),s.len(),s.as_ptr());

//     // Append some text to the string
//     s.push_str(" and some additional text");
//     println!("After update: {}", s);
//     println!("Capacity:{}, Length: {}, pointer: {:p}", s.capacity(),s.len(),s.as_ptr());
// }

// -------------------------------------------------------------------------------------------

// Ownership

// fn main() {
//     let x: i32 = 1; // crated on stack, owner is main fn
//     let y = 3; // created on stack, owner is main fn
//     println!("{}", sum(x, y));
//     println!("Hello, world!");
// }

// fn sum(a: i32, b: i32) -> i32 {
//     let c = a + b; // owner of c ,a ,b is sum fn
//     return c;
// }

// fn main(){
//     let x =String::from("Hii");
//     take_ownership(x);
//     // print!("{}", x); // can not use here because of ownership
// }
// fn take_ownership(some_string:String){ // some_string takes the ownership, x is no longer it's owner.
//     print!("{}", some_string);
// }

// fn main() {
//     let mut x = String::from("Hii");
//     x = take_ownership(x);
//     print!("{}", x); // can use here because ownership return to the caller
// }
// fn take_ownership(some_string: String) -> String {
//     // some_string takes the ownership, x is no longer it's owner.
//     print!("{}", some_string);
//     return some_string; // ownership return to the caller;
// }

// --------------------------------------------------------------------------------

// Borrowing and references

// fn main(){
//     let mut str: String = String::from("Hello ");
//     update_str(&mut str);
//     print!("{}",str);
// }

// fn update_str(str: &mut String){
// str.push_str("World");
// }

// --------------------------------------------------------------------------------

// Struct

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User{
//         active: true,
//         username: String::from("Nitin"),
//         email: String::from("hellonitin@gmail.com"),
//         sign_in_count:64,
//     };
//     println!("User 1 username: {:?} and his email id is: {:?}..he is activated {:?} times and yes it is {:?}" , user1.username, user1.email, user1.sign_in_count, user1.active);
// }

// ----------------------------------------------------------------------------------

// struct Reac{
//     width: u32,
//     height: u32,
// }

// impl Reac {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let reac = Reac {
//         width : 30,
//         height : 60,
//     };
//     println!("Rec area is {:?}", reac.area());
// }

// ---------------------------------------------------------------------------------

// Pathern Matching

// Define an enum called Shape
// enum Shape {
//     Circle(f64),
//     Square(f64),
//     Rectangle(f64, f64),
// }

// fn calcaulated_shape(shape: Shape) -> f64 {
//     match shape {
//         Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
//         Shape::Square(side_length) => side_length * side_length,
//         Shape::Rectangle(width, height) => width * height,
//     }
// }

// fn main() {
//     let circle = Shape::Circle(5.0);
//     let square = Shape::Square(4.0);
//     let rectangle = Shape::Rectangle(3.0, 6.0);

//     println!("Area of Circle {}", calcaulated_shape(circle));
//     println!("Area of square {}", calcaulated_shape(square));
//     println!("Area of rectangle {}", calcaulated_shape(rectangle));
// }

// ---------------------------------------------------------------------------------

// Borrowing

// fn main(){
//     let mut s1 = String::from("Hello");
//     {
//         let s2 = &s1;
//         println!("Before modification: s1 is {} and s2 is {}", s1, s2);
//     }

//     {
//         let s3 = &mut s1;
//         add_to_main(s3);
//     }

//     let s2 = &s1; // Reborrow s1 as s2 after modification
//     println!("After modification: s1 is {} and s2 is {}", s1, s2);
// }

// fn add_to_main(s3: &mut String){
//     s3.push_str(" world");
// }

// -----------------------------------------------------------------------------------

// Array

// fn main() {
//     let mut arr;
//     arr = [0,1,2,3,4,5,6];
//     println!("arr[0] is = {}", arr[0]);
//     println!("arr = {:?}", arr);

//     arr[1] = 30;

//     println!("arr={:?}", arr);

//     println!("length is {}", arr.len())
// }

// fn main() {
//     let arr: [&str; 3] = ["one", "two", "three"];
//     update_it(arr);
//     println!("arr={:?}", arr)
//     }

//     fn update_it(mut arr1:[&str; 3]) {
//         arr1[0] = "88";
//         println!("arr1={:?}", arr1)
// }

// fn main() {
//     let mut arr: [&str; 3] = ["one", "two", "three"];
//     update_it(&mut arr);
//     println!("arr={:?}", arr)
//     }

//     fn update_it( arr1: &mut [&str; 3]) {
//         arr1[0] = "88";
//         println!("arr1={:?}", arr1)
// }

// -------------------------------------------------------------------------------

// match

// fn main() {
//     let number = 5;
//     match number {
//         1=>println!("number is 1"),
//         2=>println!("number is 2"),
//         3=>println!("number is 3"),
//         4=>println!("number is 4"),
//         5=>println!("number is 5"),
//         6=>println!("number is 6"),
//         7=>println!("number is 7"),
//         _ => println!("number is outside the range 1-7"),

//     }
// }

// fn main() {

//     let number:i8 = 6;

//     fn is_even(num:i8)->bool{
//          num%2==0
//         }

//         match is_even(number) {
//             true=> print!("Yes it is even number"),
//             false=> print!("No it isn't even number")
//         }
// }

// ------------------------------------------------------------------------------------------------

// input / output

// use std::{self, io};

// fn main() {
//     let mut input = String::new();
//     io::stdin()
//     .read_line(&mut input)
//     .expect("failed");
//     println!("User input is: {}", input);
// }

// ------------------------------------------------------------------------------------------------

// use rand::prelude::*;
// use std::io;

// fn guess_checker(selected: &str, random: &str) -> bool {
//     selected == random
// }

// fn main() {
//     let guess_list = ["apple", "banana", "cherry", "date", "elderberry"];
//     let mut random = thread_rng();

//     let index = random.gen_range(0..guess_list.len());
//     let random_fruit = guess_list[index];

//     println!(
//         "Please select from: {}, {}, {}, {}, {}",
//         guess_list[0], guess_list[1], guess_list[2], guess_list[3], guess_list[4]
//     );

//     let mut input = String::new();
//     loop {
//         input.clear();
//         println!("Please enter the fruit name: ");

//         match io::stdin().read_line(&mut input) {
//             Ok(_) => {
//                 let selected_fruit = input.trim().to_lowercase();
//                 println!("Fruit Selected: {}", selected_fruit);

//                 if !guess_list.contains(&selected_fruit.as_str()) {
//                     println!("Fruit entered does not exist, please try again.");
//                     continue;
//                 }

//                 if guess_checker(&selected_fruit, random_fruit) {
//                     println!("You are the winner!");
//                     break;
//                 } else {
//                     println!("Incorrect guess, please try again.");
//                 }
//             }
//             Err(error) => {
//                 println!("Error: {}", error);
//             }
//         }
//     }
// }


// -------------------------------------------------------------------------------------------

// use rand::prelude::*;
// use std::io;

// fn main() {
//     let random_number = thread_rng().gen_range(1..=10);
//     let mut input = String::new();

//     println!("Guess the number between 1 and 10!");

//     loop {
//         input.clear();
//         println!("Please enter your guess: ");

//         match io::stdin().read_line(&mut input) {
//             Ok(_) => {
//                 let guess: i32 = match input.trim().parse() {
//                     Ok(num) => num,
//                     Err(_) => {
//                         println!("Please enter a valid number.");
//                         continue;
//                     }
//                 };

//                 if guess == random_number {
//                     println!("Congratulations, you guessed the right number!");
//                     break;
//                 } else {
//                     println!("Incorrect guess, please try again.");
//                 }
//             }
//             Err(error) => {
//                 println!("Error: {}", error);
//             }
//         }
//     }
// }


// -------------------------------------------------------------------------------------------

// use rand::prelude::*;
// use std::io;

// fn main() {
//     let color_list = ["red", "blue", "green", "yellow", "orange"];
//     let mut random = thread_rng();

//     let index = random.gen_range(0..color_list.len());
//     let random_color = color_list[index];

//     println!(
//         "Please select from: {}, {}, {}, {}, {}",
//         color_list[0], color_list[1], color_list[2], color_list[3], color_list[4]
//     );

//     let mut input = String::new();
//     loop {
//         input.clear();
//         println!("Please enter the color name: ");

//         match io::stdin().read_line(&mut input) {
//             Ok(_) => {
//                 let selected_color = input.trim().to_lowercase();
//                 println!("Color Selected: {}", selected_color);

//                 if !color_list.contains(&selected_color.as_str()) {
//                     println!("Color entered does not exist, please try again.");
//                     continue;
//                 }

//                 if selected_color == random_color {
//                     println!("You are the winner!");
//                     break;
//                 } else {
//                     println!("Incorrect guess, please try again.");
//                 }
//             }
//             Err(error) => {
//                 println!("Error: {}", error);
//             }
//         }
//     }
// }

// -------------------------------------------------------------------------------------------------

// use rand::prelude::*;
// use std::io;

// fn main() {
//     let animal_list = ["cat", "dog", "lion", "tiger", "elephant"];
//     let mut random = thread_rng();

//     let index = random.gen_range(0..animal_list.len());
//     let random_animal = animal_list[index];

//     println!(
//         "Please select from: {}, {}, {}, {}, {}",
//         animal_list[0], animal_list[1], animal_list[2], animal_list[3], animal_list[4]
//     );

//     let mut input = String::new();
//     loop {
//         input.clear();
//         println!("Please enter the animal name: ");

//         match io::stdin().read_line(&mut input) {
//             Ok(_) => {
//                 let selected_animal = input.trim().to_lowercase();
//                 println!("Animal Selected: {}", selected_animal);

//                 if !animal_list.contains(&selected_animal.as_str()) {
//                     println!("Animal entered does not exist, please try again.");
//                     continue;
//                 }

//                 if selected_animal == random_animal {
//                     println!("You are the winner!");
//                     break;
//                 } else {
//                     println!("Incorrect guess, please try again.");
//                 }
//             }
//             Err(error) => {
//                 println!("Error: {}", error);
//             }
//         }
//     }
// }

// ----------------------------------------------------------------------------


// use rand::Rng;
// use std::cmp::Ordering;
// use std::io; // Import io module from standard library

// fn main() {
//     println!("Guess the number!");

//     let secret_number = rand::thread_rng().gen_range(1..=100); // Generate a random number between 1 and 100 (inclusive)

//     println!("The secret number is: {}", secret_number);
//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

    
//     let guess: u32 = match guess.trim().parse() {
//         Ok(num) => num,
//         Err(_) => {
//             println!("Please enter a valid number!");
//             return; // Exit the program if parsing fails
//         }
//     };

//     println!("You guessed: {}", guess);

//     match guess.cmp(&secret_number) {
//         Ordering::Less => println!("Too small!"),
//         Ordering::Greater => println!("Too big!"),
//         Ordering::Equal => println!("You win!"),
//     }
// }

// -------------------------------------------------------------------------------------------

//  convert Celsius to Fahrenheit !!

// use std::io;

// fn main() {
//     let mut user_input = String::new();

//     io::stdin().read_line(&mut user_input).expect("Fialed to read user input");

//     let  celsius:f64 = user_input.trim().parse().expect("failed to enter valid number");

//     let  fahrenheit= celsius * 9.0/5.0 + 9.0;

//     println!("Fahrenheit is : {}" ,fahrenheit );
// }

// ------------------------------------------------------------------------------------------

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let mut user1 = User{
//         active: true,
//         username: String::from("user1"),
//         email:String::from("email1@gmail.com"),
//         sign_in_count: 1,
//     };

//     user1.email = String::from("email11@gmail.com");

//     let user2 = User{
//         active:false,
//         email: String::from("email2@gmail.com"),
//         ..user1
//     };

//     println!("User1 email address is : {}", user1.email);
//     println!("User1 email address is : {}", user2.email);
// }

// --------------------------------------------------------------------------------------------

// struct Rectangle {
//     width: u32,
//     height:  u32,
// }

// fn main() {
//     let reac1 = Rectangle {
//         width: 32,
//         height: 65,
//     };
//     println!("Area of reactangle is: {:?}", area(&reac1));
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// -----------------------------------------------------------------------------------------------------------------

// diff between struck and enums

// Let's consider IP addresses, which can be either IPv4 or IPv6. Using structs to represent this would require multiple structs and additional logic to manage the different types:

struct Ipv4Addr {
    address: String,
}

struct Ipv6Addr {
    address: String,
}

let home = IpAddr::V4(Ipv4Addr {
    address: String::from("127.0.0.1"),
});

let loopback = IpAddr::V6(Ipv6Addr {
    address: String::from("::1"),
});

// emun 

enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
