// Advanced Features

// - Unsafe Rust: how to opt out of some of Rust’s guarantees and take responsibility for manually upholding those guarantees

// - Advanced traits: associated types, default type parameters, fully qualified syntax, supertraits, and the newtype pattern in relation to traits

// - Advanced types: more about the newtype pattern, type aliases, the never type, and dynamically sized types

// - Advanced functions and closures: function pointers and returning closures

// - Macros: ways to define code that defines more code at compile time

// Unsafe Rust
// All the code we’ve discussed so far has had Rust’s memory safety guarantees enforced at compile time. However,
// Rust has a second language hidden inside it that doesn’t enforce these memory safety guarantees: it’s called
// unsafe Rust and works just like regular Rust, but gives us extra superpowers.

// Unsafe Rust exists because, by nature, static analysis is conservative. When the compiler tries to determine
// whether or not code upholds the guarantees, it’s better for it to reject some valid programs rather than accept
// some invalid programs. Although the code might be okay, as far as Rust is able to tell, it’s not! In these cases,
// you can use unsafe code to tell the compiler, “Trust me, I know what I’m doing.” The downside is that you use it
// at your own risk: if you use unsafe code incorrectly, problems due to memory unsafety, such as null pointer
// dereferencing, can occur.

// Another reason Rust has an unsafe alter ego is that the underlying computer hardware is inherently unsafe. If
// Rust didn’t let you do unsafe operations, you couldn’t do certain tasks. Rust needs to allow you to do low-level
// systems programming, such as directly interacting with the operating system or even writing your own operating
// system. Working with low-level systems programming is one of the goals of the language. Let’s explore what we
// can do with unsafe Rust and how to do it.

// Unsafe Superpowers
// To switch to unsafe Rust, use the unsafe keyword and then start a new block that holds the unsafe code. You can
// take four actions in unsafe Rust, called unsafe superpowers, that you can’t in safe Rust. Those superpowers
//include the ability to:

// - Dereference a raw pointer

// - Call an unsafe function or method

// - Access or modify a mutable static variable

// - Implement an unsafe trait

// It’s important to understand that unsafe doesn’t turn off the borrow checker or disable any other of Rust’s safety
// checks: if you use a reference in unsafe code, it will still be checked. The unsafe keyword only gives you access
// to these four features that are then not checked by the compiler for memory safety. You’ll still get some degree
// of safety inside of an unsafe block.

// To isolate unsafe code as much as possible, it’s best to enclose unsafe code within a safe abstraction and provide
// a safe API, which we’ll discuss later in the chapter when we examine unsafe functions and methods. Parts of the
// standard library are implemented as safe abstractions over unsafe code that has been audited. Wrapping unsafe code
// in a safe abstraction prevents uses of unsafe from leaking out into all the places that you or your users might
// want to use the functionality implemented with unsafe code, because using a safe abstraction is safe.

// Dereferencing a Raw Pointer
// In Chapter 4, in the “Dangling References” section, we mentioned that the compiler ensures references are always valid.
// Unsafe Rust has two new types called raw pointers that are similar to references. As with references, raw pointers can
// be immutable or mutable and are written as *const T and *mut T, respectively. The asterisk isn’t the dereference
// operator; it’s part of the type name. In the context of raw pointers, immutable means that the pointer can’t be directly
// assigned to after being dereferenced.

// Different from references and smart pointers, raw pointers:
// - Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location

// - Aren’t guaranteed to point to valid memory

// - Are allowed to be null

// - Don’t implement any automatic cleanup

// Below shows how to create an immutable and a mutable raw pointer from references. Notice that we don’t include
// the unsafe keyword in this code. We can create raw pointers in safe code; we just can’t dereference raw pointers
// outside an unsafe block, as you’ll see in a bit.
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

// Next, we’ll create a raw pointer whose validity we can’t be so certain of. Listing 19-2 shows how to create a raw
// pointer to an arbitrary location in memory. Trying to use arbitrary memory is undefined: there might be data at
// that address or there might not, the compiler might optimize the code so there is no memory access, or the program
// might error with a segmentation fault. Usually, there is no good reason to write code like this, but it is possible.
let address = 0x012345usize;
let r = address as *const i32;

// Recall that we can create raw pointers in safe code, but we can’t dereference raw pointers and read the data being
// pointed to. In Listing 19-3, we use the dereference operator * on a raw pointer that requires an unsafe block.
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}

// Here is an unsafe function named dangerous that doesn’t do anything in its body:
unsafe fn dangerous() {}

unsafe {
    dangerous();
}

// Creating a Safe Abstraction over Unsafe Code
// Just because a function contains unsafe code doesn’t mean we need to mark the entire function as unsafe. In fact,
// wrapping unsafe code in a safe function is a common abstraction. As an example, let’s study a function from the
// standard library, split_at_mut, that requires some unsafe code and explore how we might implement it. This safe
// method is defined on mutable slices: it takes one slice and makes it two by splitting the slice at the index
// given as an argument. Listing 19-4 shows how to use split_at_mut.
let mut v = vec![1, 2, 3, 4, 5, 6];
let r = &mut v[..];

let (a, b) = r.split_at_mut(3);

assert_eq!(a, &mut [1, 2, 3]);
assert_eq!(b, &mut [4, 5, 6]);

// Shows how to use an unsafe block, a raw pointer, and some calls to unsafe functions to make the implementation
// of split_at_mut work.
use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}
// Note that we don’t need to mark the resulting split_at_mut function as unsafe, and we can call this function from
// safe Rust. We’ve created a safe abstraction to the unsafe code with an implementation of the function that uses
// unsafe code in a safe way, because it creates only valid pointers from the data this function has access to.

// In contrast, the use of slice::from_raw_parts_mut in Listing 19-7 would likely crash when the slice is used. This
// code takes an arbitrary memory location and creates a slice 10,000 items long.

// We don’t own the memory at this arbitrary location, and there is no guarantee that the slice this code creates
// contains valid i32 values. Attempting to use slice as though it’s a valid slice results in undefined behavior.
use std::slice;

let address = 0x01234usize;
let r = address as *mut i32;

let slice: &[i32] = unsafe {
    slice::from_raw_parts_mut(r, 10000);
}

// Using extern Functions to Call External Code
// Sometimes, your Rust code might need to interact with code written in another language. For this, Rust has a
// keyword, extern, that facilitates the creation and use of a Foreign Function Interface (FFI). An FFI is a way
// for a programming language to define functions and enable a different (foreign) programming language to call
// those functions.

// Listing 19-8 demonstrates how to set up an integration with the abs function from the C standard library.
// Functions declared within extern blocks are always unsafe to call from Rust code. The reason is that other
// languages don’t enforce Rust’s rules and guarantees, and Rust can’t check them, so responsibility falls on
// the programmer to ensure safety.
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// Within the extern "C" block, we list the names and signatures of external functions from another language we
// want to call. The "C" part defines which application binary interface (ABI) the external function uses: the
// ABI defines how to call the function at the assembly level. The "C" ABI is the most common and follows the
// C programming language’s ABI

// Calling Rust Functions from Other Languages
// We can also use extern to create an interface that allows other languages to call Rust functions. Instead of
// an extern block, we add the extern keyword and specify the ABI to use just before the fn keyword. We also need
// to add a #[no_mangle] annotation to tell the Rust compiler not to mangle the name of this function. Mangling
// is when a compiler changes the name we’ve given a function to a different name that contains more information
// for other parts of the compilation process to consume but is less human readable. Every programming language
// compiler mangles names slightly differently, so for a Rust function to be nameable by other languages, we must
// disable the Rust compiler’s name mangling.

// In the following example, we make the call_from_c function accessible from C code, after it’s compiled to a
// shared library and linked from C:
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// Accessing or Modifying a Mutable Static Variable
// Until now, we’ve not talked about global variables, which Rust does support but can be problematic with Rust’s
// fownership rules. If two threads are accessing the same mutable global variable, it can cause a data race.

// In Rust, global variables are called static variables. Listing 19-9 shows an example declaration and use of a
// static variable with a string slice as a value.
static HELLO_WORLD: &str = "Hello, world!"

fn main() {
    println!("name is: {}", HELLO_WORLD);
}

//  The names of static variables are in SCREAMING_SNAKE_CASE by convention, and we must annotate the variable’s type,
//  which is &'static str in this example. Static variables can only store references with the 'static lifetime, which
//  means the Rust compiler can figure out the lifetime; we don’t need to annotate it explicitly. Accessing an
//  immutable static variable is safe.

// Constants and immutable static variables might seem similar, but a subtle difference is that values in a static
// variable have a fixed address in memory. Using the value will always access the same data. Constants, on the
// other hand, are allowed to duplicate their data whenever they’re used.

// Another difference between constants and static variables is that static variables can be mutable. Accessing and
// modifying mutable static variables is unsafe. Listing 19-10 shows how to declare, access, and modify a mutable
// static variable named COUNTER.
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// With mutable data that is globally accessible, it’s difficult to ensure there are no data races, which is why
// Rust considers mutable static variables to be unsafe. Where possible, it’s preferable to use the concurrency
// techniques and thread-safe smart pointers we discussed in Chapter 16 so the compiler checks that data accessed
// from different threads is done safely.

// Implementing an Unsafe Trait
// The final action that works only with unsafe is implementing an unsafe trait. A trait is unsafe when at least one
// of its methods has some invariant that the compiler can’t verify. We can declare that a trait is unsafe by adding
// the unsafe keyword before trait and marking the implementation of the trait as unsafe too, as shown below:
unsafe trait Foo {
    // methods here
}

unsafe impl Foo for i32 {
    // method implementations here
}

// A great example of a situation where this technique is useful is with operator overloading. Operator overloading is
// customizing the behavior of an operator (such as +) in particular situations.

// Rust doesn’t allow you to create your own operators or overload arbitrary operators. But you can overload the
// operations and corresponding traits listed in std::ops by implementing the traits associated with the operator.
// For example, in Listing 19-14 we overload the + operator to add two Point instances together. We do this by
// implementing the Add trait on a Point struct:
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

// The default generic type in this code is within the Add trait. Here is its definition:
trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

// This code should look generally familiar: a trait with one method and an associated type. The new part is
// RHS=Self: this syntax is called default type parameters. The RHS generic type parameter (short for “right
// hand side”) defines the type of the rhs parameter in the add method. If we don’t specify a concrete type
// for RHS when we implement the Add trait, the type of RHS will default to Self, which will be the type we’re
// implementing Add on.

// When we implemented Add for Point, we used the default for RHS because we wanted to add two Point instances.
// Let’s look at an example of implementing the Add trait where we want to customize the RHS type rather than
// using the default.

// We have two structs, Millimeters and Meters, holding values in different units. We want to add values in
// millimeters to values in meters and have the implementation of Add do the conversion correctly. We can
// implement Add for Millimeters with Meters as the RHS:
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 & 1000))
    }
}

// To add Millimeters and Meters, we specify impl Add<Meters> to set the value of the RHS type parameter instead
// of using the default of Self.

// You’ll use default type parameters in two main ways:

// - To extend a type without breaking existing code
// - To allow customization in specific cases most users won’t need

// The standard library’s Add trait is an example of the second purpose: usually, you’ll add two like types, but
// the Add trait provides the ability to customize beyond that. Using a default type parameter in the Add trait
// definition means you don’t have to specify the extra parameter most of the time. In other words, a bit of
// implementation boilerplate isn’t needed, making it easier to use the trait.

// The first purpose is similar to the second but in reverse: if you want to add a type parameter to an existing
// trait, you can give it a default to allow extension of the functionality of the trait without breaking the
// existing implementation code.

// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
// Nothing in Rust prevents a trait from having a method with the same name as another trait’s method, nor does
// Rust prevent you from implementing both traits on one type. It’s also possible to implement a method directly
// on the type with the same name as methods from traits.

// When calling methods with the same name, you’ll need to tell Rust which one you want to use. Consider the code
// in Listing 19-16 where we’ve defined two traits, Pilot and Wizard, that both have a method called fly. We then
// implement both traits on a type Human that already has a method named fly implemented on it. Each fly method
// does something different.
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// To call the fly methods from either the Pilot trait or the Wizard trait, we need to use more explicit syntax to
// specify which fly method we mean.
fn main() {
    let person = Human;

    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly(); // or Human::fly(&person)
}

struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new() -> Person {
        Person {
            name: String::from("josce"),
            age: 24,
        }
    }

    fn get_age(&self) -> u8 {
        self.age
    }
}

// This code is for an animal shelter that wants to name all puppies Spot, which is implemented
// in the baby_name associated function that is defined on Dog. The Dog type also implements
// the trait Animal, which describes characteristics that all animals have. Baby dogs are called
// puppies, and that is expressed in the implementation of the Animal trait on Dog in the baby_name
// function associated with the Animal trait.
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
}

// To disambiguate and tell Rust that we want to use the implementation of Animal for Dog, we need to use fully
// qualified syntax. Listing 19-21 demonstrates how to use fully qualified syntax.
fn main() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

// In general, fully qualified syntax is defined as follows:
<Type as Trait>::function(receiver_if_method, next_arg, ...);

// Using Supertraits to Require One Trait’s Functionality Within Another Trait
// Sometimes, you might need one trait to use another trait’s functionality. In this case, you need
// to rely on the dependent trait also being implemented. The trait you rely on is a supertrait of
// the trait you’re implementing.

// For example, let’s say we want to make an OutlinePrint trait with an outline_print method that
// will print a value framed in asterisks. That is, given a Point struct that implements Display
// to result in (x, y), when we call outline_print on a Point instance that has 1 for x and 3
// for y, it should print the following:
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// Because we’ve specified that OutlinePrint requires the Display trait, we can use the to_string
// function that is automatically implemented for any type that implements Display. If we tried
// to use to_string without adding a colon and specifying the Display trait after the trait name,
// we’d get an error saying that no method named to_string was found for the type &Self in the
// current scope.

// Let’s see what happens when we try to implement OutlinePrint on a type that doesn’t implement
// Display, such as the Point struct:
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}
// We get an error saying that Display is required but not implemented.

// To fix this, we implement Display on Point and satisfy the constraint that OutlinePrint requires, like so:
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Using the Newtype Pattern to Implement External Traits on External Types
// In Chapter 10 in the “Implementing a Trait on a Type” section, we mentioned the orphan rule that states
// we’re allowed to implement a trait on a type as long as either the trait or the type are local to our
// crate. It’s possible to get around this restriction using the newtype pattern, which involves creating
// a new type in a tuple struct. (We covered tuple structs in the “Using Tuple Structs without Named Fields
// to Create Different Types” section of Chapter 5.) The tuple struct will have one field and be a thin
// wrapper around the type we want to implement a trait for. Then the wrapper type is local to our crate,
// and we can implement the trait on the wrapper. Newtype is a term that originates from the Haskell
// programming language. There is no runtime performance penalty for using this pattern, and the wrapper
// type is elided at compile time.

// As an example, let’s say we want to implement Display on Vec<T>, which the orphan rule prevents us from
// doing directly because the Display trait and the Vec<T> type are defined outside our crate. We can make
// a Wrapper struct that holds an instance of Vec<T>; then we can implement Display on Wrapper and use the
// Vec<T> value, as shown below:
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
// The implementation of Display uses self.0 to access the inner Vec<T>, because Wrapper is a tuple struct and
// Vec<T> is the item at index 0 in the tuple. Then we can use the functionality of the Display type on Wrapper

// The downside of using this technique is that Wrapper is a new type, so it doesn’t have the methods of the value
// it’s holding. We would have to implement all the methods of Vec<T> directly on Wrapper such that the methods
// delegate to self.0, which would allow us to treat Wrapper exactly like a Vec<T>. If we wanted the new type to
// have every method the inner type has, implementing the Deref trait (discussed in Chapter 15 in the “Treating
// Smart Pointers Like Regular References with the Deref Trait” section) on the Wrapper to return the inner type
// would be a solution. If we don’t want the Wrapper type to have all the methods of the inner type—for example,
// to restrict the Wrapper type’s behavior—we would have to implement just the methods we do want manually.

// Creating Type Synonyms with Type Aliases
// Along with the newtype pattern, Rust provides the ability to declare a type alias to give an existing type
// another name. For this we use the type keyword. For example, we can create the alias Kilometers to i32 like so:
type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x + y);

// Because Kilometers and i32 are the same type, we can add values of both types and we can pass Kilometers values
// to functions that take i32 parameters. However, using this method, we don’t get the type checking benefits that
// we get from the newtype pattern discussed earlier.

// The main use case for type synonyms is to reduce repetition. For example, we might have a lengthy type like this:
Box<dyn Fn() + Send + 'static>

// Writing this lengthy type in function signatures and as type annotations all over the code can be tiresome and
// error prone. Imagine having a project full of code like that:
let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
    // --snip--
}

fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    // --snip--
}

// A type alias makes this code more manageable by reducing the repetition. In Listing 19-25, we’ve introduced an
// alias named Thunk for the verbose type and can replace all uses of the type with the shorter alias Thunk.
let f: Thunk = Box::new(|| println!("hey"));

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
}

// Type aliases are also commonly used with the Result<T, E> type for reducing repetition. Consider the std::io
// module in the standard library. I/O operations often return a Result<T, E> to handle situations when operations
// fail to work. This library has a std::io::Error struct that represents all possible I/O errors. Many of the
// functions in std::io will be returning Result<T, E> where the E is std::io::Error, such as these functions
// in the Write trait:
use std::io::Error;
use std::fmt;

type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
// The type alias helps in two ways: it makes code easier to write and it gives us a consistent interface across
// all of std::io. Because it’s an alias, it’s just another Result<T, E>, which means we can use any methods that
// work on Result<T, E> with it, as well as special syntax like the ? operator.

// The Never Type that Never Returns
// Rust has a special type named ! that’s known in type theory lingo as the empty type because it has no values.
// We prefer to call it the never type because it stands in the place of the return type when a function will
// never return. Here is an example:
fn bar() -> ! {
    // --snip--
}
// This code is read as “the function bar returns never.” Functions that return never are called diverging functions.
// We can’t create values of the type ! so bar can never possibly return.

// But what use is a type you can never create values for?
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

// At the time, we skipped over some details in this code. In Chapter 6 in “The match Control Flow Operator” section,
// we discussed that match arms must all return the same type. So, for example, the following code doesn’t work:
let guess = match guess.trim().parse() {
    Ok(_) => 5,
    Err(_) => "hello",
};

// The type of guess in this code would have to be an integer and a string, and Rust requires that guess have only
// one type. So what does continue return? How were we allowed to return a u32 from one arm and have another arm
// that ends with continue in Listing 19-26?

// As you might have guessed, continue has a ! value. That is, when Rust computes the type of guess, it looks at
// both match arms, the former with a value of u32 and the latter with a ! value. Because ! can never have a value,
// Rust decides that the type of guess is u32.

// The formal way of describing this behavior is that expressions of type ! can be coerced into any other type.
// We’re allowed to end this match arm with continue because continue doesn’t return a value; instead, it moves
// control back to the top of the loop, so in the Err case, we never assign a value to guess.

// The never type is useful with the panic! macro as well. Remember the unwrap function that we call on Option<T>
// values to produce a value or panic? Here is its definition:
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}

// In this code, the same thing happens as in the match in Listing 19-26: Rust sees that val has the type T and panic!
// has the type !, so the result of the overall match expression is T. This code works because panic! doesn’t produce
// a value; it ends the program. In the None case, we won’t be returning a value from unwrap, so this code is valid.

// One final expression that has the type ! is a loop:
print!("forever ");

loop {
    print!("and ever ");
}

// Here, the loop never ends, so ! is the value of the expression. However, this wouldn’t be true if we included a
// break, because the loop would terminate when it got to the break.

// Dynamically Sized Types and the Sized Trait
// Due to Rust’s need to know certain details, such as how much space to allocate for a value of a particular type,
// there is a corner of its type system that can be confusing: the concept of dynamically sized types. Sometimes
// referred to as DSTs or unsized types, these types let us write code using values whose size we can know only
// at runtime.

// To work with DSTs, Rust has a particular trait called the Sized trait to determine whether or not a type’s
// size is known at compile time. This trait is automatically implemented for everything whose size is known
// at compile time. In addition, Rust implicitly adds a bound on Sized to every generic function. That is, a
// generic function definition like this:
fn generic<T>(t: T) {
    // --snip--
}

// is actually treated as though we had written this:
fn generic<T: Sized>(t: T) {
    // --snip--
}

// By default, generic functions will work only on types that have a known size at compile time. However, you can
// use the following special syntax to relax this restriction:
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}

// A trait bound on ?Sized is the opposite of a trait bound on Sized: we would read this as “T may or may not be
// Sized.” This syntax is only available for Sized, not any other traits.

// Also note that we switched the type of the t parameter from T to &T. Because the type might not be Sized, we
// need to use it behind some kind of pointer. In this case, we’ve chosen a reference.

// Advanced Functions and Closures
// Function Pointers
// We’ve talked about how to pass closures to functions; you can also pass regular functions to functions! This
// technique is useful when you want to pass a function you’ve already defined rather than defining a new closure.
// Doing this with function pointers will allow you to use functions as arguments to other functions. Functions
// coerce to the type fn (with a lowercase f), not to be confused with the Fn closure trait. The fn type is called
// a function pointer. The syntax for specifying that a parameter is a function pointer is similar to that of
// closures

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}

// As an example of where you could use either a closure defined inline or a named function, let’s look at a use
// of map. To use the map function to turn a vector of numbers into a vector of strings, we could use a closure,
// like this:
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(|i| i.to_string())
    .collect();

// Or we could name a function as the argument to map instead of the closure, like this:
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(ToString::to_string)
    .collect();

// We have another useful pattern that exploits an implementation detail of tuple structs and tuple-struct enum
// variants. These types use () as initializer syntax, which looks like a function call. The initializers are
// actually implemented as functions returning an instance that’s constructed from their arguments. We can use
// these initializer functions as function pointers that implement the closure traits, which means we can specify
// the initializer functions as arguments for methods that take closures, like so:
enum Status {
    Value(u32),
    Stop,
}

let list_of_statuses: Vec<Status> = (0u32..20)
    .map(Status::Value)
    .collect();

// And returning a closure:
fn returns_closure() -> Box<dyn Fn(32) -> i32> {
    Box::new(|x| x + 1)
}

// Macros
// We’ve used macros like println! throughout this book, but we haven’t fully explored what a macro is and
// how it works. The term macro refers to a family of features in Rust: declarative macros with macro_rules!
// and three kinds of procedural macros:

// Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
// Attribute-like macros that define custom attributes usable on any item
// Function-like macros that look like function calls but operate on the tokens specified as their argument

// The Difference Between Macros and Functions
// Fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming.
// In Appendix C, we discuss the derive attribute, which generates an implementation of various traits for you.
// We’ve also used the println! and vec! macros throughout the book. All of these macros expand to produce more
// code than the code you’ve written manually.

// Metaprogramming is useful for reducing the amount of code you have to write and maintain, which is also one
// of the roles of functions. However, macros have some additional powers that functions don’t.

// A function signature must declare the number and type of parameters the function has. Macros, on the other
// hand, can take a variable number of parameters: we can call println!("hello") with one argument or
// println!("hello {}", name) with two arguments. Also, macros are expanded before the compiler interprets
// the meaning of the code, so a macro can, for example, implement a trait on a given type. A function can’t,
// because it gets called at runtime and a trait needs to be implemented at compile time.

// The downside to implementing a macro instead of a function is that macro definitions are more complex than
// function definitions because you’re writing Rust code that writes Rust code. Due to this indirection, macro
// definitions are generally more difficult to read, understand, and maintain than function definitions.

// Another important difference between macros and functions is that you must define macros or bring them into
// scope before you call them in a file, as opposed to functions you can define anywhere and call anywhere.

// Declarative Macros with macro_rules! for General Metaprogramming
// The most widely used form of macros in Rust is declarative macros. These are also sometimes referred to as
// “macros by example,” “macro_rules! macros,” or just plain “macros.” At their core, declarative macros allow
// you to write something similar to a Rust match expression. As discussed in Chapter 6, match expressions are
// control structures that take an expression, compare the resulting value of the expression to patterns, and
// then run the code associated with the matching pattern. Macros also compare a value to patterns that are
// associated with particular code: in this situation, the value is the literal Rust source code passed to the
// macro; the patterns are compared with the structure of that source code; and the code associated with each
// pattern, when matched, replaces the code passed to the macro. This all happens during compilation.

// To define a macro, you use the macro_rules! construct. Let’s explore how to use macro_rules! by looking at
// how the vec! macro is defined. Chapter 8 covered how we can use the vec! macro to create a new vector with
// particular values. For example, the following macro creates a new vector containing three integers:
let v: Vec<u32> = vec![1, 2, 3];

// Here is a slightly simplified definition of the vec! macro. Note: The actual definition of the vec! macro
// in the standard library includes code to preallocate the correct amount of memory up front. That code is an
// optimization that we don’t include here to make the example simpler.
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    }
}

// The #[macro_export] annotation indicates that this macro should be made available whenever the crate in which
// the macro is defined is brought into scope. Without this annotation, the macro can’t be brought into scope.

// First, a set of parentheses encompasses the whole pattern. A dollar sign ($) is next, followed by a set of
// parentheses that captures values that match the pattern within the parentheses for use in the replacement code.
// Within $() is $x:expr, which matches any Rust expression and gives the expression the name $x.

// The comma following $() indicates that a literal comma separator character could optionally appear after the
// code that matches the code in $(). The * specifies that the pattern matches zero or more of whatever precedes
// the *.

// When we call this macro with vec![1, 2, 3];, the $x pattern matches three times with the three expressions 1,
// 2, and 3.

// Now let’s look at the pattern in the body of the code associated with this arm: temp_vec.push() within $()* is
// generated for each part that matches $() in the pattern zero or more times depending on how many times the pattern
// matches. The $x is replaced with each expression matched. When we call this macro with vec![1, 2, 3];, the code
// generated that replaces this macro call will be the following:
let mut temp_vec = Vec::new();
temp_vec.push(1);
temp_vec.push(2);
temp_vec.push(3);
temp_vec

// Procedural Macros for Generating Code from Attributes
// The second form of macros is procedural macros, which act more like functions (and are a type of procedure).
// Procedural macros accept some code as an input, operate on that code, and produce some code as an output rather
// than matching against patterns and replacing the code with other code as declarative macros do.
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {}

// The function that defines a procedural macro takes a TokenStream as an input and produces a TokenStream as an
// output. The TokenStream type is defined by the proc_macro crate that is included with Rust and represents a sequence
// of tokens. This is the core of the macro: the source code that the macro is operating on makes up the input
// TokenStream, and the code the macro produces is the output TokenStream. The function also has an attribute attached
// to it that specifies which kind of procedural macro we’re creating. We can have multiple kinds of procedural macros
// in the same crate.

// How to Write a Custom derive Macro
// Let’s create a crate named hello_macro that defines a trait named HelloMacro with one associated function named
// hello_macro. Rather than making our crate users implement the HelloMacro trait for each of their types, we’ll
// provide a procedural macro so users can annotate their type with #[derive(HelloMacro)] to get a default
// implementation of the hello_macro function. The default implementation will print Hello, Macro! My name is
// TypeName! where TypeName is the name of the type on which this trait has been defined. In other words, we’ll
// write a crate that enables another programmer to write code like Listing 19-30 using our crate.

pub trait HelloMacro {
    fn hello_macro();
}

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}

// The next step is to define the procedural macro. At the time of this writing, procedural macros need to be in
// their own crate. Eventually, this restriction might be lifted. The convention for structuring crates and macro
// crates is as follows: for a crate named foo, a custom derive procedural macro crate is called foo_derive. Let’s
// start a new crate called hello_macro_derive inside our hello_macro project:
$ cargo new hello_macro_derive --lib

// hello_macro_derive/Cargo.toml
[lib]
proc-macro = true

[dependencies]
syn = "0.14.4"
quote = "0.6.3"

// hello_macro_derive/src/lib.rs
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

// hello_macro_derive/src/lib.rs
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}

// Attribute-like macros
// Attribute-like macros are similar to custom derive macros, but instead of generating code for the derive attribute,
// they allow you to create new attributes. They’re also more flexible: derive only works for structs and enums; attributes
// can be applied to other items as well, such as functions. Here’s an example of using an attribute-like macro: say you
// have an attribute named route that annotates functions when using a web application framework:
#[route(GET, "/")]
fn index() {}

// This #[route] attribute would be defined by the framework as a procedural macro. The signature of the macro definition
// function would look like this:
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}

// Function-like macros
// Function-like macros define macros that look like function calls. Similarly to macro_rules! macros, they’re more
// flexible than functions; for example, they can take an unknown number of arguments. However, macro_rules! macros
// can be defined only using the match-like syntax we discussed in the section “Declarative Macros with macro_rules!
// for General Metaprogramming” earlier. Function-like macros take a TokenStream parameter and their definition manipulates
// that TokenStream using Rust code as the other two types of procedural macros do. An example of a function-like macro
// is an sql! macro that might be called like so:
let sql = sql!(SELECT * FROM posts WHERE id=1);

// This macro would parse the SQL statement inside it and check that it’s syntactically correct, which is much more
// complex processing than a macro_rules! macro can do. The sql! macro would be defined like this:
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {}