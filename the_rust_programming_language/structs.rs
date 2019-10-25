// To define a struct, we enter the keyword struct and name the entire struct. A struct’s name should
// describe the significance of the pieces of data being grouped together. Then, inside curly brackets,
// we define the names and types of the pieces of data, which we call fields. For example, Listing 5-1
// shows a struct that stores information about a user account.

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

// Uses Field Init Shorthand when variable and field have the same name
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Using struct update syntax which specifies that the remaining fields not explicitly set should have
// the same value as the fields in the given instance.

// So instead of this:
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};

// You can do this
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

// In the User struct definition on line 6, we used the owned String type rather than the &str string slice type.
// This is a deliberate choice because we want instances of this struct to own all of its data and for that data to be
// valid for as long as the entire struct is valid. It’s possible for structs to store references to data owned by
// something else, but to do so requires the use of lifetimes. Lifetimes ensure that the data referenced by a struct
// is valid for as long as the struct is. Let’s say you try to store a reference in a struct without specifying
// lifetimes, like this, which won’t work:

struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}

// Example program:
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// A refactor of the above program using tuples:
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// A refactor of the program using structs:
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// With special annotation to opt in to debug. This allows us to print the Rectangle struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {:?}", rect1);
}

// Methods

// Methods are similar to functions: they’re declared with the fn keyword and their name, they can
// have parameters and a return value, and they contain some code that is run when they’re called
// from somewhere else. However, methods are different from functions in that they’re defined within
// the context of a struct (or an enum or a trait object, which we cover in Chapters 6 and 17,
// respectively), and their first parameter is always self, which represents the instance of the
// struct the method is being called on.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

// Associated Functions
// Can have multiple impl blocks. This one shows one where the first argument is not self.
// These are called associated functions and are called like Rectange::square(). They do
// not need instances of the struct. (Sort of a static method)
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}