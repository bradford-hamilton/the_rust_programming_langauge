fn main() {
    println!("Hello, world!");

    // constant with snake case integer for clarity
    const MAX_POINTS: u32 = 100_000;

    // shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    // Integer Types
    // Length	    Signed	  Unsigned
    // 8-bit	    i8	      u8
    // 16-bit	    i16	      u16
    // 32-bit	    i32    	  u32
    // 64-bit	    i64   	  u64
    // 128-bit    i128	    u128
    // arch	      size	    usize

    // When developing, rust will panic during an integer overflow. However, in "prod"
    // (--release flag) rust performs two’s complement wrapping. Ex: when you have a
    // u8 you will go to 0 after 255 -> wraps back around.

    // Number literals	  Example
    // Decimal	          98_222
    // Hex	              0xff
    // Octal	            0o77
    // Binary	            0b1111_0000
    // Byte (u8 only)	    b'A'

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // Booleans
    let t = true;
    let f: bool = false; // with explicit type annotation

    // Char literal
    let c = 'z';

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    // Arrays
    // Every element must be the same type.
    // Have a fixed length
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // Or
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // Or
    let a = [3; 5]; // [3, 3, 3, 3, 3]

    // Index access
    let first = a[0];
    let second = a[1];

    // When you attempt to access an element using indexing, Rust will check that
    // the index you’ve specified is less than the array length. If the index is
    // greater than or equal to the array length, Rust will panic.

    // Functions
    fn some_function() {
        another_function(5, 6);
    }

    fn another_function(x: i32, y: i32) {
        println!("The value of x is: {}", x);
        println!("The value of y is: {}", y);
    }
}
