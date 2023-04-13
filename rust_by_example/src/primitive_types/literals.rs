pub fn example() {
    //Interger addition
    println!("1 + 2 = {}", 1u32 + 2);

    //Integer Subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // Try changing 1i32 to 1u32

    //Scientic notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    //Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    //Bitwise operation
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32);

    //use underscores to imporver readability!
    println!("One million is written as {}", 1_000_000u32);
}
