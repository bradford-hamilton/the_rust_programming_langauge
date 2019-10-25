// How to Write Tests
// To change a function into a test function, add #[test] on the line before fn. When you run your tests with the cargo test
// command, Rust builds a test runner binary that runs the functions annotated with the test attribute and reports on whether
// each test function passes or fails.

// First `cargo new adder --lib`
// Then in src/lib.rs:
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}

// For now, let’s ignore the top two lines and focus on the function to see how it works. Note the #[test] annotation before the
// fn line: this attribute indicates this is a test function, so the test runner knows to treat this function as a test. We could
// also have non-test functions in the tests module to help set up common scenarios or perform common operations, so we need to
// indicate which functions are tests by using the #[test] attribute.

// Checking Results with the assert! Macro
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 6, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 6, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }
}

// Testing Equality with the assert_eq! and assert_ne! Macros
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(6, add_two(4));
    }
}

// Note that in some languages and test frameworks, the parameters to the functions that assert two values are equal are called
// expected and actual, and the order in which we specify the arguments matters. However, in Rust, they’re called left and right,
// and the order in which we specify the value we expect and the value that the code under test produces doesn’t matter. We could
// write the assertion in this test as assert_eq!(add_two(2), 4), which would result in a failure message that displays assertion
// failed: `(left == right)` and that left was 5 and right was 4.

// The assert_ne! macro will pass if the two values we give it are not equal and fail if they’re equal. This macro is most useful
// for cases when we’re not sure what a value will be, but we know what the value definitely won’t be if our code is functioning
// as we intend. For example, if we’re testing a function that is guaranteed to change its input in some way, but the way in which
// the input is changed depends on the day of the week that we run our tests, the best thing to assert might be that the output of
// the function is not equal to the input.

// Under the surface, the assert_eq! and assert_ne! macros use the operators == and !=, respectively. When the assertions fail,
// these macros print their arguments using debug formatting, which means the values being compared must implement the PartialEq
// and Debug traits. All the primitive types and most of the standard library types implement these traits. For structs and enums
// that you define, you’ll need to implement PartialEq to assert that values of those types are equal or not equal. You’ll need
// to implement Debug to print the values when the assertion fails. Because both traits are derivable traits, as mentioned in
// Listing 5-12 in Chapter 5, this is usually as straightforward as adding the #[derive(PartialEq, Debug)] annotation to your
// struct or enum definition. See Appendix C, “Derivable Traits,” for more details about these and other derivable traits.

// Adding Custom Failure Messages
// For example, let’s say we have a function that greets people by name and we want to test that the name we pass into the
// function appears in the output:
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}

// This result just indicates that the assertion failed and which line the assertion is on. A more useful failure message in this
// case would print the value we got from the greeting function. Let’s change the test function, giving it a custom failure message
// made from a format string with a placeholder filled in with the actual value we got from the greeting function:
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`", result
    );
}

// Checking for Panics with should_panic
// In addition to checking that our code returns the correct values we expect, it’s also important to check that our code handles
// error conditions as we expect. For example, consider the Guess type that we created in Chapter 9, Listing 9-10. Other code that
// uses Guess depends on the guarantee that Guess instances will contain only values between 1 and 100. We can write a test that
// ensures that attempting to create a Guess instance with a value outside that range panics.

// Going back to write & test Guess from earlier in the book:
pub struct Guest {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}

// Tests that use should_panic can be imprecise because they only indicate that the code has caused some panic. A should_panic test
// would pass even if the test panics for a different reason from the one we were expecting to happen. To make should_panic tests
// more precise, we can add an optional expected parameter to the should_panic attribute. The test harness will make sure that the
// failure message contains the provided text.
pub struct Guest {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

// Using Result<T, E> in Tests. The it_works function now has a return type, Result<(), String>. In the body of the function, rather
// than calling the assert_eq! macro, we return Ok(()) when the test passes and an Err with a String inside when the test fails.
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

// Controlling How Tests Are Run
// Some command line options go to cargo test, and some go to the resulting test binary. To separate these two types of arguments, you
// list the arguments that go to cargo test followed by the separator -- and then the ones that go to the test binary. Running cargo test --help
// displays the options you can use with cargo test, and running cargo test -- --help displays the options you can use after the separator --.

// If you don’t want to run the tests in parallel or if you want more fine-grained control over the number of threads used, you can send the
// --test-threads flag and the number of threads you want to use to the test binary. Take a look at the following example:
$ cargo test -- --test-threads=1

// By default, if a test passes, Rust’s test library captures anything printed to standard output. For example, if we call println! in a test and
// the test passes, we won’t see the println! output in the terminal; we’ll see only the line that indicates the test passed. If a test fails,
// we’ll see whatever was printed to standard output with the rest of the failure message.

// If we want to see printed values for passing tests as well, we can disable the output capture behavior by using the --nocapture flag:
$ cargo test -- --nocapture

// Running a Subset of Tests by Name
// If we had the following code, we could test everything at once or specifically a single test:
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

}

// We could run just the one_hundred test like so:
$ cargo test one_hundred

// You can specify just part of a name to match for all tests that match:
cargo test add // tests both add_two_and_two and add_three_and_two

// Ignoring Some Tests Unless Specifically Requested
// Sometimes a few specific tests can be very time-consuming to execute, so you might want to exclude them during most runs of cargo test. Rather
// than listing as arguments all tests you do want to run, you can instead annotate the time-consuming tests using the ignore attribute to exclude
// them, as shown here:
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
// After #[test] we add the #[ignore] line to the test we want to exclude. Now when we run our tests, it_works runs, but expensive_test doesn’t:

// Now you can test ignored tests with:
$ cargo test -- --ignored

// Test Organization
// As mentioned at the start of the chapter, testing is a complex discipline, and different people use different terminology and organization. The
// Rust community thinks about tests in terms of two main categories: unit tests and integration tests. Unit tests are small and more focused,
// testing one module in isolation at a time, and can test private interfaces. Integration tests are entirely external to your library and use
// your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test.

// Unit Tests
// The purpose of unit tests is to test each unit of code in isolation from the rest of the code to quickly pinpoint where code is and isn’t working
// as expected. You’ll put unit tests in the src directory in each file with the code that they’re testing. The convention is to create a module named
// tests in each file to contain the test functions and to annotate the module with cfg(test).

// The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test, not when you run cargo build.
// This saves compile time when you only want to build the library and saves space in the resulting compiled artifact because the tests are not included.
// You’ll see that because integration tests go in a different directory, they don’t need the #[cfg(test)] annotation. However, because unit tests go in
// the same files as the code, you’ll use #[cfg(test)] to specify that they shouldn’t be included in the compiled result.

// Testing Private Functions
// There’s debate within the testing community about whether or not private functions should be tested directly, and other languages make it difficult or
// impossible to test private functions. Regardless of which testing ideology you adhere to, Rust’s privacy rules do allow you to test private functions.
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
// Note that the internal_adder function is not marked as pub, but because tests are just Rust code and the tests module is just another module, you
// can bring internal_adder into a test’s scope and call it. If you don’t think private functions should be tested, there’s nothing in Rust that will
// compel you to do so.

// Integration Tests
// In Rust, integration tests are entirely external to your library. They use your library in the same way any other code would, which means they can
// only call functions that are part of your library’s public API. Their purpose is to test whether many parts of your library work together correctly.
// Units of code that work correctly on their own could have problems when integrated, so test coverage of the integrated code is important as well.
// To create integration tests, you first need a tests directory.

// The tests Directory
// We create a tests directory at the top level of our project directory, next to src. Cargo knows to look for integration test files in this directory.
// We can then make as many test files as we want to in this directory, and Cargo will compile each of the files as an individual crate.

// tests/integration_test.rs
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}

// We’ve added use adder at the top of the code, which we didn’t need in the unit tests. The reason is that each test in the tests directory is a separate
// crate, so we need to bring our library into each test crate’s scope.

// Submodules in Integration Tests
// As you add more integration tests, you might want to make more than one file in the tests directory to help organize them; for example, you can group the
// test functions by the functionality they’re testing. As mentioned earlier, each file in the tests directory is compiled as its own separate crate.

// Treating each integration test file as its own crate is useful to create separate scopes that are more like the way end users will be using your crate.
// However, this means files in the tests directory don’t share the same behavior as files in src do, as you learned in Chapter 7 regarding how to
// separate code into modules and files.

// Common:

// tests/common.rs
pub fn setup() {
    // setup code specific to your library's tests would go here
}

// To avoid having common appear in the test output, instead of creating tests/common.rs, we’ll create tests/common/mod.rs. This is an alternate
// naming convention that Rust also understands. Naming the file this way tells Rust not to treat the common module as an integration test file.
// When we move the setup function code into tests/common/mod.rs and delete the tests/common.rs file, the section in the test output will no longer
// appear. Files in subdirectories of the tests directory don’t get compiled as separate crates or have sections in the test output.

// After we’ve created tests/common/mod.rs, we can use it from any of the integration test files as a module. Here’s an example of calling the setup
// function from the it_adds_two test in tests/integration_test.rs:

// tests/integration_test.rs
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

// Integration Tests for Binary Crates
// If our project is a binary crate that only contains a src/main.rs file and doesn’t have a src/lib.rs file, we can’t create integration tests
// in the tests directory and bring functions defined in the src/main.rs file into scope with a use statement. Only library crates expose
// functions that other crates can use; binary crates are meant to be run on their own.

// This is one of the reasons Rust projects that provide a binary have a straightforward src/main.rs file that calls logic that lives in the
// src/lib.rs file. Using that structure, integration tests can test the library crate with use to make the important functionality available.
// If the important functionality works, the small amount of code in the src/main.rs file will work as well, and that small amount of code
// doesn’t need to be tested.
