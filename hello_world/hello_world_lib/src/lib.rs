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
        // If the name of the element is the same as the name of the variable, you can just write it alone like that
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

///let's define another function that handles straight line graph formula ///Assuminng that m, c and x have to be passed.
///Here you can use a normal function.
///Below, we have to use array slice as x, otherwise, we will need to specify a size.

fn straight_line_function(m: i32, c: i32, xses: &[i32]) -> Vec<(i32, i32)> {
    let mut output: Vec<(i32, i32)> = Vec::new(); //you could also use vec![] to bring in initial arguments
    for x in xses {
        let y = (m * x) + c;
        output.push((*x, y)) //here we have to dereference the borrowed x, to get the value
    }
    output
}

pub fn run6() {
    let mut x = 10;

    println!("x before change = {}", x);

    let y = &mut x; //y is a mutable reference to x
    let z: *const u32 = y; // z is an immutable raw pointer to y which references x
                           //let  a = y as *mut u32; a is a mutable raw pointer to y which references x
    let a: *mut u32 = y; //a is a mutable raw pointer to y which references x

    println!("y={:?}", y); //expect value in x
    println!("z={:?}", z); //expect memory address
    println!("a={:?}", a); //expect same memory address as z above

    *y = 11; //expect value in x to change
    println!("x after first change = {}", x);

    unsafe {
        *a = 12; //expect value in x to change
        assert!(x == 12)
    };

    println!("x after second chage = {}", x);
}

pub fn run7() {
    //Error handling
    // panic!("You called panic");

    //Illustraction Some
    let mut v = vec!["a", "b", "c"];

    //pop an element from the vector
    let x = v.pop();

    //if by your business value, you are sure that vec contains values i.e Option is Some, use unwrap to bring out the value
    // expect does thesame thing as unwrap then it also provides an error message should incase there is no value to unwrap
    println!("{}", x.unwrap());
    println!("{}", x.expect("I expected a value from my vec"));

    //what if we know there's a possiblity of having no Some value but we don't want to crash if we don't get a value
    match x {
        Some(value) => println!("we've got a value"),
        None => println!("Your vector is empty"),
    }

    //compare above to
    let mut v2: Vec<&str> = Vec::new();

    //continue from phone
    //let mut y2 = v2.pop().unwrap(); //will panic without message because it ...
    //let mut y2 = v2.pop().expect("Do not call pop on an empty Vector"); //...

    //Exercis: How can you ensure that your program does not panic when you ...
    let y2 = match v2.pop() {
        Some(val) => val,
        None => "Empty vector",
    };
    println!("{}", y2);

    //let's use ? for option
    let mut v3 = vec![1, 2, 3];

    //Closures(read up more)
    //   - don't have to have names
    //   - within a given scope where you define a function, that func cannot use variable defined
    //     outside it unless they were passed to it. Closure can do this
    let mut plus_one = || -> Option<i32> { Some(v3.pop()? + 1) };

    println!("Plus one {}", plus_one().unwrap())
}

//Let's see Result instead of Option
//Here it returns OK value vs Err, unlike Option that returns Some value

//Adjust the following to return Result
pub fn multiplier(numbers: &[f64]) -> f64 {
    let mut product = 1f64;
    for n in numbers {
        product = product * n;
    }
    product
}

//what if we want to return Err to the caller of this functio when less
//than two arguments are passed

#[derive(Debug)]
pub struct ErrorTypes {
    pub number: u8,
    pub message: &'static str,
    pub detail: &'static str,
}

//Let's create static variables for error types
const INVALID_ARG_LEN_ERR: ErrorTypes = ErrorTypes {
    number: 101,
    message: "Invalid Arg Length",
    detail: "Two or more arguments are expected",
};

const INVALID_ARG_TYPE_ERR: ErrorTypes = ErrorTypes {
    number: 102,
    message: "Invalid Arg tYPE. f64 expected",
    detail: "Invalid Arg Type. f64 expected. You must convert your arg to f64",
};

//mature multiplier
pub fn mature_multiplier(numbers: &[f64]) -> Result<f64, ErrorTypes> {
    if numbers.len() < 2 {
        return Err(INVALID_ARG_LEN_ERR);
    };
    let mut product = 1f64;
    for n in numbers {
        product *= n;
    }
    Ok(product)
}

pub fn run8() {
    {
        //s is not vaid here, it's not yet declared
        let s = "hello"; //s is valid from the point forward

        //do stuff with s
    } //this scope is now over, and s is no longer valid

    //above so far is all about stack. What about when we are dealing with
    //heap allocated memory where for memory saving purposes, different variable can
    //share the heap allocated memory?

    {
        let s = String::from("hello"); //s is valid from this point forward

        //do stuff with s
    } //this scope is now over, and s is no longer valid

    /*
    THere is a natural point at which we can return the memoryour
    String needs to the allocator: when s goes out of scope.
     */
}

#[macro_export] //in-built in Rust
macro_rules! rectangles {
    ($($rect_props_tuple:expr), *) => {
        //I want to return a Vector of Rectangles
        {
            let mut rect_vec = Vec::new();
            //take our expression received, get the length, width and name
            //from each and create the appropriate Rect and push each
            //to our rect_rec

            $(let (length, width, name) = $rect_props_tuple;
            let rect = Rect {
                length: length as f32,
                width: width as f32,
                name: name as &str,
            };
            rect_vec.push(rect);
        )*
        rect_vec
        }
    };
}

//Try out our rectangle! declarative macro.
pub fn run9() {
    let rects = rectangles!((1, 1, "rect1"), (3.5, 7.0, "rect2"));
    println!(
        "Area of rectangle 1 = {}; area of rectangle 2 = {}",
        rects[0].area(),
        rects[1].area()
    )
}

//You can also have multiple expressions in a declarative macro.
//What if you want a second expression that contains defaults for
//length, width and name for the rect
//This implies that length, width and name will be optional.
#[macro_export]
macro_rules! rectangles_with_default {
    (($($rect_props_tuple:expr), *), $default_tuple:expr) => {
        {
            let mut rect_vec = Vec::new();
            let (default_length, default_width, default_name) = $default_tuple;
            $(
                let (length, width, name) = $rect_props_tuple;
                let rect = Rect{
                    length: if length.is_none() { default_length as f32}else{ length.unwrap() as f32},
                    width: if width.is_none(){ default_width as f32 }else{ width.unwrap() as f32},
                    name: if name.is_none(){ default_name as &str }else{ name.unwrap() as &str}
                };
                rect_vec.push(rect);
            )*
            rect_vec
        }
    };
}

pub fn run10() {
    let rects = rectangles_with_default!(
        (
            (None as Option<u32>, Some(-1), Some("rect1")),
            (Some(3.5), Some(7.0), None as Option<&str>)
        ),
        (1, 1, "default name")
    );

    println!(
        "Area of rectangle 1 = {}; area of rectangle 2 = {}",
        rects[0].area(),
        rects[1].area()
    )
}

pub fn run11() {
    use rect_shape::Shape;
    use rect_shape_derive::Shape;

    #[derive(Debug, Clone, Shape)]
    struct RectWithDerivedShape {
        length: f32,
        width: f32,
        name: &'static str,
    }

    //the Shape trait implementations
    //should be available for RectWithDerivedShape
    //without further explicit implementation

    let rectangle1 = RectWithDerivedShape {
        length: 1.0,
        width: 2.0,
        name: "Rect 1 with derived Shape trait",
    };
    println!(
        "Area of rectangle1 with derived Shape = {}",
        rectangle1.area()
    );
}
