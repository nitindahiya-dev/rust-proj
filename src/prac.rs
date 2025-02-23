// #[derive(Debug)]  // Enable debug formatting
// struct Rect {
//     height: f32,
//     width: f32,
// }

// pub fn mainly() {
//     let r = Rect {
//         width: 10.0,
//         height: 10.0,
//     };

//     println!("Debug: {:?}", r.height * r.width);  // Debug: Rect { height: 10.0, width: 10.0 }

// }

// ------------------------------------------------------------------------------------------------
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }

// pub fn mainly() {
//     let direction = Direction::North;

//     steer(direction); // Pass the direction variable to the steer function
// }

// fn steer(dir: Direction) {
//     match dir {
//         Direction::East => println!("East Direction"),
//         Direction::North => println!("North Direction"),
//         Direction::West => println!("West Direction"),
//         Direction::South => println!("South Direction"),
//     }
// }

// ------------------------------------------------------------------------------------------------
// enum Shape {
//     Square(f32),
//     Circle(f32),
//     Rectangle(f32, f32),
// }

// pub fn mainly() {
//     let shape = Shape::Square(10.0);
//     let shape_circle = Shape::Circle(10.0);
//     let shape_rectangle = Shape::Rectangle(10.0, 20.0);

//     area(shape);
//     area(shape_circle);
//     area(shape_rectangle);
// }

// fn area(shape: Shape) {
//     match shape {
//         Shape::Square(side) => {
//             let area = side * side;
//             println!("Area of the square: {}", area);
//         }
//         Shape::Circle(radius) => {
//             let area = std::f32::consts::PI * radius * radius;
//             println!("Area of the circle: {}", area);
//         }
//         Shape::Rectangle(width, height) => {
//             let area = width * height;
//             println!("Area of the rectangle: {}", area);
//         }
//     }
// }

// {Using Impl}

// enum Shape {
//     Square(f32),
//     Circle(f32),
//     Rectangle(f32, f32),
// }

// impl Shape {
//     // Method to calculate the area of the shape
//     fn area(&self) -> f32 {
//         match self {
//             Shape::Square(side) => side * side,
//             Shape::Circle(radius) => std::f32::consts::PI * radius * radius,
//             Shape::Rectangle(width, height) => width * height,
//         }
//     }
// }

// pub fn mainly() {
//     let shape = Shape::Square(10.0);
//     let shape_circle = Shape::Circle(10.0);
//     let shape_rectangle = Shape::Rectangle(10.0, 20.0);

//     // Call the area method on each shape
//     println!("Area of the square: {}", shape.area());
//     println!("Area of the circle: {}", shape_circle.area());
//     println!("Area of the rectangle: {}", shape_rectangle.area());
// }

// ------------------------------------------------------------------------------------------------

// Option and Result Trait

// enum Reault<T, E> {
//     Ok(T),
//     Err(E),
// }

// enum Option<T> {
//     Some(T),
//     None,
// }

use std::fs;

pub fn mainly() {
    // Use read_to_string to read a file, not read_dir (which is for directories)
    let result = fs::read_to_string("a.txt");
    
    match result {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error: {}", e),
    }
}

