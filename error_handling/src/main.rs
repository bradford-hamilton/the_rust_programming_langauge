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
    },
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