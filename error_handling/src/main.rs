// Panic macro
panic!("crash and burn");

// Recoverable Errors with Result
// Most errors aren’t serious enough to require the program to stop entirely. Sometimes, when a function fails, it’s
// for a reason that you can easily interpret and respond to. For example, if you try to open a file and that
// operation fails because the file doesn’t exist, you might want to create the file instead of terminating the
// process.
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// The T and E are generic type parameters: we’ll discuss generics in more detail in Chapter 10. What you need to
// know right now is that T represents the type of the value that will be returned in a success case within the Ok
// variant, and E represents the type of the error that will be returned in a failure case within the Err variant.
// Because Result has these generic type parameters, we can use the Result type and the functions that the standard
// library has defined on it in many different situations where the successful value and error value we want to
// return may differ.
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
        }
    };
}

// Note that, like the Option enum, the Result enum and its variants have been brought into scope by the prelude, so
// we don’t need to specify Result:: before the Ok and Err variants in the match arms.

// The code in Listing 9-4 will panic! no matter why File::open failed. What we want to do instead is take different
// actions for different failure reasons: if File::open failed because the file doesn’t exist, we want to create the
// file and return the handle to the new file. If File::open failed for any other reason—for example, because we didn’t
// have permission to open the file—we still want the code to panic! in the same way as it did in Listing 9-4. Look at
// Listing 9-5, which adds an inner match expression.
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

// A more seasoned rust dev may write something more like:
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

// For example, Listing 9-6 shows a function that reads a username from a file. If the file doesn’t exist or can’t be
// read, this function will return those errors to the code that called this function.

use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File.open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// A Shortcut for Propagating Errors: the ? Operator
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}

// And with chaining methods
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s);

    Ok(s)
}

// Reading a file into a string is a fairly common operation, so Rust provides the convenient fs::read_to_string function
// that opens the file, creates a new String, reads the contents of the file, puts the contents into that String, and
// returns it. Of course, using fs::read_to_string doesn’t give us the opportunity to explain all the error handling, so
// we did it the longer way first.
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// The main function is special, and there are restrictions on what its return type must be. One valid return type for main
// is (), and conveniently, another valid return type is Result<T, E>, as shown here:
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn, Error>> {
  let f = File::open("hello.txt")?;

  Ok(())
}
// The Box<dyn Error> type is called a trait object, which we’ll talk about in the “Using Trait Objects that Allow for Values
// of Different Types” section in Chapter 17. For now, you can read Box<dyn Error> to mean “any kind of error.” Using ? in a
// main function with this return type is allowed.