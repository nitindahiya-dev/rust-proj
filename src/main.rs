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

fn main(){
    let mut str: String = String::from("Hello ");
    update_str(&mut str);
    print!("{}",str);
}

fn update_str(str: &mut String){
str.push_str("World");
}