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

//let the string element be on the heap
trait Shape2 {
    fn area(&self) -> f32;
    // fn new(length: f32, width: f32, name: String) -> Self;
    fn get_length(&self) -> f32;
    fn set_length(&mut self, length: f32);
    fn get_width(&self) -> f32;
    fn set_width(&mut self, width: f32);
    fn get_name(&self) -> &str;
    fn set_name(&mut self, name: &str);
}

// Derive an implementation of default triat
#[derive(Debug, Clone)]

struct Rect2 {
    length: f32,
    width: f32,
    name: String, // on heap
}

impl Rect2 {
    fn default() -> Self {
        Rect2 {
            length: 1f32,
            width: 1f32,
            // name: "default_name".to_string(),
            name: String::from("default_name"), // name: "default_name".to_owned(), //at a time, this was better and more used than to_string..

                                                //slicing is taking a full string or a string that you can get a part of it, meanin that you want it static. to_string() and to_owned() converts string slice to a heap

                                                //Demanding for memory for a string is to create a string for it. The compiler would then provide a pointer to that string.

                                                //If someone else wants it, they will borrow it. If they want to own it, they can to to_owned or create a new String using new string or string from.
        }
    }
}

impl Shape2 for Rect2 {
    ///Associated function used to create a new shape
    /// Not  amethod because it is not taking in self

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

    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_name(
        &mut self,
        name: &str, //it can slice/get from anywhere
    ) {
        self.name = name.to_owned();
    }
}

///Implement a From trait for Rect2 that takes a string slice with the format "length, width, name"

impl From<&str> for Rect2 {
    fn from(s: &str) -> Self {
        //when you are treversing an array .next(), you are making changing because you are changing the pointer so therefore it should be mutable
        let mut parts = s.split(',');
        let length = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0f32,
        };

        let width = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0f32,
        };

        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };

        Rect2 {
            length,
            width,
            name: name.to_owned(),
        }
    }
    //Essentially, we are supposed to tokenize the string and extract the three parts into respective variables.
    //Then use them to constitute a new Rect2
}

//Implement Into trait for Rect2
impl Into<String> for Rect2 {
    // this requires the heap because you are returning String
    fn into(self) -> String {
        //Let's return a string template literal

        //format macro can have the features of print macro but not to specifically print to anywhere
        format!("My name is {} and my area is {}.", self.name, self.area())

        //Print macro is to print to the console or to anywhere
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

    // ..rectangle1 means that it takes all the data already present in rectangle1
    let rectangle4 = Rect { ..rectangle1 };

    //here it takes all the data in rectangle1 except for length, which is assigned here
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

//Let's demand that our structs be created on the heap ... as we spin a new struct
//We are going to use a smat pointer to put it in the heap
// The smart pointer is called Box
pub fn run2() {
    let mut rectangle1 = Box::new(Rect2 {
        length: 12f32,
        width: 9f32,
        name: "Rectangle 1".to_owned(),
    });

    //To handle the Shape2 as well (which is a trait)
    println!("{}", rectangle1.area());

    let rectangle2 = Rect2::from("20.0,30.0,Rectangle2");
    let rectangle3: Rect2 = "25.0,37.0,Rectangle3".into(); //this is the reverse of From

    let s: String = rectangle3.into(); //this is the Info<String> trait impl. Explicit type declaration. This is a move
    println!("About me: {}", s);
    // println!("Area of Rectangle 3 = {}", rectangle3.area());

    println!("Rectangle 1 = {:#?}", rectangle1);
    println!("Area of Rectangle 2 = {}", rectangle2.area());
}

//Functions and Clourses

//In Rust, functions have their own types

//function to add 2 integers
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn apply(
    f: fn(i32, i32) -> i32, //this is a parameter that is a function that returns a value of type i32
    x: i32,
    y: i32,
) -> i32 {
    f(x, y) //here, the parameters x and y are passed into whatever function is passed into f().
}

pub fn run3() {
    let f = add;
    let x = 7;
    let y = 8;
    let z = apply(f, x, y);
    println!("The result of applying f to {} and {} is {}", x, y, z);
}
