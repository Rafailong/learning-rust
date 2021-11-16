// FIRST ATTEMPT
// fn main() {
//     let width1 = 30;
//     let height1 = 50;
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }
//
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// IMPROVEMENT USING A TUPLE
// fn main() {
//     let rect1 = (30, 50);
//
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }
//
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// USING STRUCTS
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // method
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// function
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

fn main() {
    let rect = Rectangle {
        width: 40,
        height: 60,
    };

    println!("rect is '{:?}'", rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        // area(&rect)
        rect.area()
    );
}