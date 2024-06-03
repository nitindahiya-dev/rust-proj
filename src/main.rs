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

fn main() {
    run_loop();
}

fn run_loop() {
    let mut x = Vec::new();
    for i in 0..100 {
        x.push(i);
    }
    return  println!("{:?}",x) ; 
}
