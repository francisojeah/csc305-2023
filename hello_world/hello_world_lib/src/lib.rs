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
    fn get_length(&self) -> f32;
    fn set_length(&mut self, length: f32);
    fn get_width(&self) -> f32;
    fn set_width(&mut self, width: f32);
    fn get_name(&self) -> &'static str;
    fn set_name(&mut self, name: &'static str);
}

#[derive(
    // Default,
    Debug,
    Clone,
    Copy,
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

    fn get_length(&self) -> f32 {
        self.length
    }

    fn set_length(&mut self, length: f32) {
        self.length = length;
    }

    fn get_width(&self) -> f32 {
        self.width
    }

    fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    fn get_name(&self) -> &'static str {
        self.name
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }
}

impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        // self.length == other.length && self.width == other.width && self.name == other.name;
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

pub fn run() {
    let rectangle1: Rect = Rect {
        length: 2.4,
        width: 6.3,
        name: "Rectangle",
    };

    let mut rectangle2: Rect = Rect::default();
    rectangle2.set_length(10f32);
    rectangle2.set_width(5f32);

    let rectangle3 = rectangle1.clone();

    let rectangle4 = Rect { ..rectangle1 };

    let rectangle5 = Rect {
        length: 12f32,
        ..rectangle1
    };

    println!("rectangle 1 is {:#?}", rectangle1);
    println!("rectangle 2 is {:#?}", rectangle2);
    println!(
        "Area of rectangle 1 is {}",
        format!("{:.2?}", rectangle1.area())
    );

    assert_eq!(rectangle1, rectangle3);
    println!("If you can see this, your two triangles are equal");
}
