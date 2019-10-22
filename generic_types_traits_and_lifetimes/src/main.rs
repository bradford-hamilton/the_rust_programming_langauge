// Every programming language has tools for effectively handling the duplication of concepts. In Rust, one such tool
// is generics. Generics are abstract stand-ins for concrete types or other properties. When we’re writing code, we
// can express the behavior of generics or how they relate to other generics without knowing what will be in their
// place when compiling and running the code. Similar to the way a function takes parameters with unknown values to
// run the same code on multiple concrete values, functions can take parameters of some generic type instead of a
// concrete type, like i32 or String. In fact, we’ve already used generics in Chapter 6 with Option<T>, Chapter 8
// with Vec<T> and HashMap<K, V>, and Chapter 9 with Result<T, E>.

// First, we’ll review how to extract a function to reduce code duplication. Next, we’ll use the same technique to make
// a generic function from two functions that differ only in the types of their parameters. We’ll also explain how to
// use generic types in struct and enum definitions.

// Then you’ll learn how to use traits to define behavior in a generic way. You can combine traits with generic types
// to constrain a generic type to only those types that have a particular behavior, as opposed to just any type.

// Finally, we’ll discuss lifetimes, a variety of generics that give the compiler information about how references
// relate to each other. Lifetimes allow us to borrow values in many situations while still enabling the compiler to
// check that the references are valid.

// Before diving into generics syntax, let’s first look at how to remove duplication that doesn’t involve generic types
// by extracting a function. Then we’ll apply this technique to extract a generic function! In the same way that you
// recognize duplicated code to extract into a function, you’ll start to recognize duplicated code that can use generics.
// Consider a short program that finds the largest number in a list:

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is: {}", largest);
}

// We *could* get the largest number in multiple lists through duplication:
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
// but that would be silly

// So what about this;
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
}

// We can use generics to create definitions for items like function signatures or structs, which we can then use
// with many different concrete data types. Let’s first look at how to define functions, structs, enums, and
// methods using generics. Then we’ll discuss how generics affect code performance.

// Let's look at an example of this functionality with a list of chars as well:
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
}

// To define a Point struct where x and y are both generics but could have different types, we can use multiple generic
// type parameters. For example, in Listing 10-8, we can change the definition of Point to be generic over types T and
// U where x is of type T and y is of type U.
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.2, y: 10.3 };
    let both_integer_and_float = Point { x: 5, y: 5.5 };
}

// In Enum definitions: As we did with structs, we can define enums to hold generic data types in their variants. Let’s
// take another look at the Option<T> enum that the standard library provides, which we used in Chapter 6:
enum Option<T> {
    Some(T),
    None,
}

// Enums can use multiple generic types as well. The definition of the Result enum that we used before is one example.
// The Result enum is generic over two types, T and E, and has two variants: Ok, which holds a value of type T, and Err,
// which holds a value of type E. This definition makes it convenient to use the Result enum anywhere we have an operation
// that might succeed (return a value of some type T) or fail (return an error of some type E). In fact, this is what we
// used to open a file in Listing 9-3, where T was filled in with the type std::fs::File when the file was opened successfully
// and E was filled in with the type std::io::Error when there were problems opening the file.
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// We can implement methods on structs and enums and use generic types in their definitions, too. This exapmle shows the
// Point<T> struct we defined in before with a method named x implemented on it.
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
// Note that we have to declare T just after impl so we can use it to specify that we’re implementing methods on the type
// Point<T>. By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in Point is
// a generic type rather than a concrete type.

// We could, for example, implement methods only on Point<f32> instances rather than on Point<T> instances with any generic
// type. Here we use the concrete type f32, meaning we don’t declare any types after impl. This code means the type Point<f32>
// will have a method named distance_from_origin and other instances of Point<T> where T is not of type f32 will not have this
// method defined. The method measures how far our point is from the point at coordinates (0.0, 0.0) and uses mathematical
// operations that are available only for floating point types.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2), self.y.powi(2)).sqrt()
    }
}

// Generic type parameters in a struct definition aren’t always the same as those you use in that struct’s method signatures. For
// example, below defines the method mixup on the Point<T, U> struct from before. The method takes another Point as a parameter,
// which might have different types from the self Point we’re calling mixup on. The method creates a new Point instance with the
// x value from the self Point (of type T) and the y value from the passed-in Point (of type W).
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y); // p3.x = 5, p3.y = c
}
// The purpose of this example is to demonstrate a situation in which some generic parameters are declared with impl and some are
// declared with the method definition. Here, the generic parameters T and U are declared after impl, because they go with the struct
// definition. The generic parameters V and W are declared after fn mixup, because they’re only relevant to the method.

// Performance of generic types -> You might be wondering whether there is a runtime cost when you’re using generic type parameters.
// The good news is that Rust implements generics in such a way that your code doesn’t run any slower using generic types than it would
// with concrete types.

// Rust accomplishes this by performing monomorphization of the code that is using generics at compile time. Monomorphization is the process
// of turning generic code into specific code by filling in the concrete types that are used when compiled.

// In this process, the compiler does the opposite of the steps we used to create the generic function in Listing 10-5: the compiler looks at
// all the places where generic code is called and generates code for the concrete types the generic code is called with. Let’s look at how
// this works with an example that uses the standard library’s Option<T> enum:
let integer = Some(5);
let float = Some(5.0);
// When Rust compiles this code, it performs monomorphization. During that process, the compiler reads the values that have been used in
// Option<T> instances and identifies two kinds of Option<T>: one is i32 and the other is f64. As such, it expands the generic definition
// of Option<T> into Option_i32 and Option_f64, thereby replacing the generic definition with the specific ones.

// The monomorphized version of the code looks like the following. The generic Option<T> is replaced with the specific definitions created
// by the compiler:
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}

// Traits: Defining Shared Behavior
// A trait tells the Rust compiler about functionality a particular type has and can share with other types. We can use traits to define
// shared behavior in an abstract way. We can use trait bounds to specify that a generic can be any type that has certain behavior. Traits
// are similar to a feature often called interfaces in other languages, although with some differences.

// A type’s behavior consists of the methods we can call on that type. Different types share the same behavior if we can call the same methods
// on all of those types. Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish
// some purpose. For example, let’s say we have multiple structs that hold various kinds and amounts of text: a NewsArticle struct that holds
// a news story filed in a particular location and a Tweet that can have at most 280 characters along with metadata that indicates whether it was
// a new tweet, a retweet, or a reply to another tweet.

// We want to make a media aggregator library that can display summaries of data that might be stored in a NewsArticle or Tweet instance. To do
// this, we need a summary from each type, and we need to request that summary by calling a summarize method on an instance. Listing 10-12 shows
// the definition of a Summary trait that expresses this behavior.
pub trait Summary {
    fn summerize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summerize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summerize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
// Implementing a trait on a type is similar to implementing regular methods. The difference is that after impl, we put the trait name that we
// want to implement, then use the for keyword, and then specify the name of the type we want to implement the trait for. Within the impl block,
// we put the method signatures that the trait definition has defined. Instead of adding a semicolon after each signature, we use curly brackets
// and fill in the method body with the specific behavior that we want the methods of the trait to have for the particular type.

// Example showing how the above would be called (no different then a normal method):
let tweet = Tweet {
  username: String::from("horse_ebooks"),
  content: String::from("of course, as you probably already know, people"),
  reply: false,
  retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());

// One restriction to note with trait implementations is that we can implement a trait on a type only if either the trait or the type is local to
// our crate. For example, we can implement standard library traits like Display on a custom type like Tweet as part of our aggregator crate
// functionality, because the type Tweet is local to our aggregator crate. We can also implement Summary on Vec<T> in our aggregator crate, because
// the trait Summary is local to our aggregator crate.

// But we can’t implement external traits on external types. For example, we can’t implement the Display trait on Vec<T> within our aggregator crate,
// because Display and Vec<T> are defined in the standard library and aren’t local to our aggregator crate. This restriction is part of a property of
// programs called coherence, and more specifically the orphan rule, so named because the parent type is not present. This rule ensures that other
// people’s code can’t break your code and vice versa. Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t
// know which implementation to use.

// Default implementations
// Sometimes it’s useful to have default behavior for some or all of the methods in a trait instead of requiring implementations for all methods on every
// type. Then, as we implement the trait on a particular type, we can keep or override each method’s default behavior.

// The below example shows how to specify a default string for the summarize method of the Summary trait instead of only defining the method signature,
// as we did in before.
pub trait Summary {
    fn summerize(&self) -> String {
        String::from("(Read more...)");
    }
}
// To use a default implementation to summarize instances of NewsArticle instead of defining a custom implementation, we specify an empty impl block with
impl Summary for NewsArticle {}

// Even though we’re no longer defining the summarize method on NewsArticle directly, we’ve provided a default implementation and specified that NewsArticle
// implements the Summary trait. As a result, we can still call the summarize method on an instance of NewsArticle, like this:
let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
};
println!("New article available! {}", article.summarize());

// Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation. In this way, a trait
// can provide a lot of useful functionality and only require implementors to specify a small part of it. For example, we could define the Summary trait to
// have a summarize_author method whose implementation is required, and then define a summarize method that has a default implementation that calls the
// summarize_author method:
pub trait Summary {
    fn summerize_author(&self) -> String;
    fn summerize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summerize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// After we define summarize_author, we can call summarize on instances of the Tweet struct, and the default implementation of summarize will call the
// definition of summarize_author that we’ve provided. Because we’ve implemented summarize_author, the Summary trait has given us the behavior of the
// summarize method without requiring us to write any more code.

// Traits as Parameters
// Previously, we implemented the Summary trait on the NewsArticle and Tweet types. We can define a notify function that calls the summarize method on its
// item parameter, which is of some type that implements the Summary trait. To do this, we can use the impl Trait syntax, like this:
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name. This parameter accepts any type that implements the
// specified trait. In the body of notify, we can call any methods on item that come from the Summary trait, such as summarize. We can call notify and pass
// in any instance of NewsArticle or Tweet. Code that calls the function with any other type, such as a String or an i32, won’t compile because those types
// don’t implement Summary.

// The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form, which is called a trait bound; it looks like this:
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// The impl Trait syntax is convenient and makes for more concise code in simple cases. The trait bound syntax can express more complexity in other cases. For
// example, we can have two parameters that implement Summary. Using the impl Trait syntax looks like this:
pub fn notify(item1: impl Summary, item2: impl Summary) {}

// If we wanted this function to allow item1 and item2 to have different types, using impl Trait would be appropriate (as long as both types implement Summary).
// If we wanted to force both parameters to have the same type, that’s only possible to express using a trait bound, like this:
pub fn notify<T: Summary>(item1: T, item2: T) {}

// We can also specify more than one trait bound. Say we wanted notify to use display formatting on item as well as the summarize method: we specify in the notify
// definition that item must implement both Display and Summary. We can do so using the + syntax:
pub fn notify(item: impl Summary + Display) {}

// Or
pub fn notify<T: Summary + Display>(item: T) {}

// Using too many trait bounds has its downsides. Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of trait
// bound information between the function’s name and its parameter list, making the function signature hard to read. For this reason, Rust has alternate syntax for
// specifying trait bounds inside a where clause after the function signature. So instead of writing this:
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}

// We can do this:
fn some_function<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("function body");
}

// Returning Types that Implement Traits.
// We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait, as shown here:
fn returns_summerizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
// By using impl Summary for the return type, we specify that the returns_summarizable function returns some type that implements the Summary trait without naming
// the concrete type. In this case, returns_summarizable returns a Tweet, but the code calling this function doesn’t know that. The ability to return a type that
// is only specified by the trait it implements is especially useful in the context of closures and iterators, which we cover in Chapter 13. Closures and iterators
// create types that only the compiler knows or types that are very long to specify. The impl Trait syntax lets you concisely specify that a function returns some
// type that implements the Iterator trait without needing to write out a very long type.

// However, you can only use impl Trait if you’re returning a single type. For example, this code that returns either a NewsArticle or a Tweet with the return type
// specified as impl Summary wouldn’t work. Does not compile:
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best
            hockey team in the NHL."),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}

// With our non-generic versions of the largest function, we were only trying to find the largest i32 or char. As discussed in the “Stack-Only Data: Copy” section
// in Chapter 4, types like i32 and char that have a known size can be stored on the stack, so they implement the Copy trait. But when we made the largest function
// generic, it became possible for the list parameter to have types in it that don’t implement the Copy trait. Consequently, we wouldn’t be able to move the value]
// out of list[0] and into the largest variable, resulting in an error.

// To call this code with only those types that implement the Copy trait, we can add Copy to the trait bounds of T! Below shows the complete code of a generic
// largest function that will compile as long as the types of the values in the slice that we pass into the function implement the PartialOrd and Copy traits,
// like i32 and char do.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&number_list);
}

// If we don’t want to restrict the largest function to the types that implement the Copy trait, we could specify that T has the trait bound Clone instead of Copy.
// Then we could clone each value in the slice when we want the largest function to have ownership. Using the clone function means we’re potentially making more
// heap allocations in the case of types that own heap data like String, and heap allocations can be slow if we’re working with large amounts of data.

// Another way we could implement largest is for the function to return a reference to a T value in the slice. If we change the return type to &T instead of T,
// thereby changing the body of the function to return a reference, we wouldn’t need the Clone or Copy trait bounds and we could avoid heap allocations.

// Using Trait Bounds to Conditionally Implement Methods
// By using a trait bound with an impl block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits.
// For example, the type Pair<T> in Listing 10-16 always implements the new function. But Pair<T> only implements the cmp_display method if its inner type T implements
// the PartialOrd trait that enables comparison and the Display trait that enables printing.
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are
// called blanket implementations and are extensively used in the Rust standard library. For example, the standard library implements the ToString trait on any type
// that implements the Display trait. The impl block in the standard library looks similar to this code:
impl<T: Display> ToString for T {
    // ...
}

// Because the standard library has this blanket implementation, we can call the to_string method defined by the ToString trait on any type that implements the Display
// trait. For example, we can turn integers into their corresponding String values like this because integers implement Display.
let s = 3.to_string();

// Traits and trait bounds let us write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to
// have particular behavior. The compiler can then use the trait bound information to check that all the concrete types used with our code provide the correct behavior.
// In dynamically typed languages, we would get an error at runtime if we called a method on a type that the type didn’t implement. But Rust moves these errors to compile
// time so we’re forced to fix the problems before our code is even able to run. Additionally, we don’t have to write code that checks for behavior at runtime because
// we’ve already checked at compile time. Doing so improves performance without having to give up the flexibility of generics.

// Another kind of generic that we’ve already been using is called lifetimes. Rather than ensuring that a type has the behavior we want, lifetimes ensure that references
// are valid as long as we need them to be. Let’s look at how lifetimes do that.

// Validating References with Lifetimes
// One detail we didn’t discuss in the “References and Borrowing” section in Chapter 4 is that every reference in Rust has a lifetime, which is the scope for which that
// reference is valid. Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred. We must annotate types when multiple types
// are possible. In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways. Rust requires us to annotate the
// relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

// The concept of lifetimes is somewhat different from tools in other programming languages, arguably making lifetimes Rust’s most distinctive feature. Although we won’t
// cover lifetimes in their entirety in this chapter, we’ll discuss common ways you might encounter lifetime syntax so you can become familiar with the concepts.