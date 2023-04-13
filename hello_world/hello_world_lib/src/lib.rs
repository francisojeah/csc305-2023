pub fn greeting_from_lib() -> String {
    let message = String::from("Hello from lib");
    println!("{}", message);
    message
}

// polymorphism === interface
// trait ===interface in rust

trait Shape {
    fn area(&self) -> f32;
    fn new(length: f32, width: f32, name: &'static str) -> Self;
}

#[derive(
    // Default,
    Debug,
)]
struct Rect {
    length: f32,
    width: f32,
    name: &'static str,
}

impl Rect {
    fn default() -> Self {
        Rect {
            length: 1f32,
            width: 1f32,
            name: "default_name",
        }
    }
}

impl Shape for Rect {
    ///Associated function used to create a new shape
    /// Not  amethod because it is not taking in self

    fn new(length: f32, width: f32, name: &'static str) -> Self {
        // If the anme of the element is the same as the name of the variable, you can just write it alone like that
        Rect {
            length,
            width,
            name,
        }
    }

    ///Area method
    fn area(&self) -> f32 {
        self.length * self.width
    }
}

pub fn run() {
    let rectangle1: Rect = Rect {
        length: 2.4,
        width: 6.3,
        name: "Rectangle",
    };
    let rectangle2: Rect = Rect::default();
    println!("rectangle 1 is {:#?}", rectangle1);
    println!("rectangle 2 is {:#?}", rectangle2);
    println!(
        "Area of rectangle 1 is {}",
        format!("{:.2?}", rectangle1.area())
    );
}
