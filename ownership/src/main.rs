fn main() {
    // Ownership is Rust’s most unique feature, and it enables Rust to make memory
    // safety guarantees without needing a garbage collector.

    // All programs have to manage the way they use a computer’s memory while running. Some
    // languages have garbage collection that constantly looks for no longer used memory as
    // the program runs; in other languages, the programmer must explicitly allocate and free
    // the memory. Rust uses a third approach: memory is managed through a system of ownership
    // with a set of rules that the compiler checks at compile time. None of the ownership
    // features slow down your program while it’s running.

    // First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:
    // - Each value in Rust has a variable that’s called its owner.
    // - There can only be one owner at a time.
    // - When the owner goes out of scope, the value will be dropped.

    // The variable s refers to a string literal, where the value of the string is hardcoded into the text of our
    // program. The variable is valid from the point at which it’s declared until the end of the current scope.
    let s = "hello";

    // Another example
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward
                         // do stuff with s
    }
    // this scope is now over, and s is no longer valid

    // To illustrate the rules of ownership, we need a data type that is more complex than anything
    // above that we've been playing with. All the types above are all stored on the stack and popped
    // off the stack when their scope is over, but we want to look at data that is stored on the heap
    // and explore how Rust knows when to clean up that data.

    // You can create a String from a string literal using the from function, like so:
    // The double colon (::) is an operator that allows us to namespace this particular from
    // function under the String type rather than using some sort of name like string_from
    // This type of string *can* be mutated:
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    // With the String type, in order to support a mutable, growable piece of text, we need to allocate
    // an amount of memory on the heap, unknown at compile time, to hold the contents. This means: The memory
    // must be requested from the operating system at runtime. We need a way of returning this memory to
    // the operating system when we’re done with our String.

    // That first part is done by us: when we call String::from, its implementation requests the
    // memory it needs. This is pretty much universal in programming languages.

    // However, the second part is different. In languages with a garbage collector (GC), the GC keeps
    // track and cleans up memory that isn’t being used anymore, and we don’t need to think about it.
    // Without a GC, it’s our responsibility to identify when memory is no longer being used and call
    // code to explicitly return it, just as we did to request it. Doing this correctly has historically
    // been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early,
    // we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly
    // one allocate with exactly one free.

    // Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.
    // Here’s a version of our scope example from Listing 4-1 using a String instead of a string literal:
    {
        let s = String::from("hello"); // s is valid from this point forward
                                       // do stuff with s
    }
    // this scope is now over, and s is no longer valid

    // There is a natural point at which we can return the memory our String needs to the operating
    // system: when s goes out of scope. When a variable goes out of scope, Rust calls a special
    // function for us. This function is called drop, and it’s where the author of String can put
    // the code to return the memory. Rust calls drop automatically at the closing curly bracket.

    // To ensure memory safety, there’s one more detail to what happens in this situation in Rust.
    // Instead of trying to copy the allocated memory, Rust considers s1 to no longer be valid and,
    // therefore, Rust doesn’t need to free anything when s1 goes out of scope. Check out what
    // happens when you try to use s1 after s2 is created; it won’t work:

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1); // Will not compile
                                // Because s2 gets assigned to s1 and s1 is no longer valid, Rust calls this a "move"
                                // In this scenario s1 was moved to s2

    // If we do want to deeply copy the heap data of the String, not just the stack data,
    // we can use a common method called clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    // Types such as integers that have a known size at compile time are stored entirely on the stack, so
    // copies of the actual values are quick to make. That means there’s no reason we would want to
    // prevent x from being valid after we create the variable y. In other words, there’s no difference
    // between deep and shallow copying here, so calling clone wouldn’t do anything different from the
    // usual shallow copying and we can leave it out.
    let x = 5;
    let y = x;

    // Types that are copy
    // - All the integer types, such as u32.
    // - The Boolean type, bool, with values true and false.
    // - All the floating point types, such as f64.
    // - The character type, char.
    // - Tuples, if they only contain types that are also Copy. For example, (i32, i32) is Copy, but (i32, String) is not.

    /******************************************************** Basic ownership example ********************************************************/
    fn main() {
        let s = String::from("hello"); // s comes into scope
        takes_ownership(s); // s's value moves into the function and so is no longer valid here

        let x = 5; // x comes into scope
        makes_copy(x); // x would move into the function, but i32 is Copy, so it’s okay to still use x afterward
    }
    // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

    fn takes_ownership(some_string: String) {
        // some_string comes into scope
        println!("{}", some_string);
    }
    // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

    fn makes_copy(some_integer: i32) {
        // some_integer comes into scope
        println!("{}", some_integer);
    }
    // Here, some_integer goes out of scope. Nothing special happens.

    /**************************************************** Another Basic ownership example ****************************************************/
    fn main() {
        let s1 = gives_ownership(); // gives_ownership moves its return value into s1
        let s2 = String::from("hello"); // s2 comes into scope
        let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    }
    // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was moved, so nothing happens. s1 goes out of scope and is dropped.

    fn gives_ownership() -> String {
        // gives_ownership will move its return value into the function that calls it
        let some_string = String::from("hello"); // some_string comes into scope
        some_string // some_string is returned and moves out to the calling function
    }

    // takes_and_gives_back will take a String and return one
    fn takes_and_gives_back(a_string: String) -> String {
        // a_string comes into scope
        a_string // a_string is returned and moves out to the calling function
    }

    /******************************************************* Returning a tuple example *******************************************************/
    fn main() {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
    }

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String
        (s, length)
    }

    // These ampersands are references, and they allow you to refer to some value without taking ownership of it
    fn main() {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    // We call having references as function parameters borrowing. As in real life, if a person owns something, you can borrow it
    // from them. When you’re done, you have to give it back. So what happens if we try to modify something we’re borrowing? Try
    // the code in Listing 4-6. Spoiler alert: it doesn’t work!
    // This code does not compile!
    fn main() {
        let s = String::from("hello");
        change(&s);
    }

    fn change(some_string: &String) {
        some_string.push_str(", world");
    }

    // However we can make s mutable as well as the reference to s shown below and this will work fine
    fn main() {
        let mut s = String::from("hello");
        change(&mut s);
    }

    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    // You can have only one mutable reference to a particular piece of data in a particular scope. This code will fail:
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;

    // Whew! We also cannot have a mutable reference while we have an immutable one. Users of an immutable reference
    // don’t expect the values to suddenly change out from under them! However, multiple immutable references are
    // okay because no one who is just reading the data has the ability to affect anyone else’s reading of the data.

    // This won't compile because it deallocates "s" as soon as the function finishes
    fn dangle() -> &String {
        // dangle returns a reference to a String
        let s = String::from("hello"); // s is a new String

        &s // we return a reference to the String, s
    }
    // Here, s goes out of scope, and is dropped. Its memory goes away. Danger!

    // Instead transfer ownership ->
    fn no_dangle() -> String {
        let s = String::from("hello");
        s
    }

    // String slices

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    // Array slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    // The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time.
    // The Rust language gives you control over your memory usage in the same way as other systems programming
    // languages, but having the owner of data automatically clean up that data when the owner goes out of
    // scope means you don’t have to write and debug extra code to get this control. Ownership affects how
    // lots of other parts of Rust work, so we’ll talk about these concepts further throughout the rest of
    // the book. Let’s move on to Chapter 5 and look at grouping pieces of data together in a struct.
}
