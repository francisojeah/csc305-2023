///associate greetings module with this crate
mod greetings;

extern crate hello_world_lib;

///Optionally oad each member of greetings
/*use greetings::default_greeting;
use greetings::spanish;
use greetings::french;*/
///Alternatively, use * to load them all
//use greetings::*;
///Alternatively, load them in one line
use greetings::{default_greeting, french, spanish};
use hello_world_lib::greeting_from_lib;
fn main() {
    println!("Hello, world!");
    println!("{}", default_greeting());
    println!("{}", spanish::default_greeting());
    println!("{}", french::default_greeting());
    println!("{}", greeting_from_lib());

    // Variables can be type annotated
    let logical = true;

    let a_float = 1.0; //Regular annotation
    let an_integer = 5i32;

    // Or a default will be used
    let default_float = 3.0;
    let default_integer = 7;
}
