// - Packages: A Cargo feature that lets you build, test, and share crates
// - Crates: A tree of modules that produces a library or executable
// - Modules and use: Let you control the organization, scope, and privacy of paths
// - Paths: A way of naming an item, such as a struct, function, or module

// The first parts of the module system we’ll cover are packages and crates. A crate is a
// binary or library. The crate root is a source file that the Rust compiler starts from
// and makes up the root module of your crate. A package is one or more crates that provide
// a set of functionality. A package contains a Cargo.toml file that describes how to build
// those crates.

//  A package must contain zero or one library crates, and no more. It can contain as many binary
//  crates as you’d like, but it must contain at least one crate (either library or binary).

// When we entered the command, Cargo created a Cargo.toml file, giving us a package. Looking at
// the contents of Cargo.toml, there’s no mention of src/main.rs because Cargo follows a convention
// that src/main.rs is the crate root of a binary crate with the same name as the package. Likewise,
// Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate
// with the same name as the package, and src/lib.rs is its crate root. Cargo passes the crate root
// files to rustc to build the library or binary.

// Here, we have a package that only contains src/main.rs, meaning it only contains a binary crate named
// my-project. If a package contains src/main.rs and src/lib.rs, it has two crates: a library and a binary,
// both with the same name as the package. A package can have multiple binary crates by placing files in
// the src/bin directory: each file will be a separate binary crate.

// In this section, we’ll talk about modules and other parts of the module system, namely paths that allow
// you to name items; the use keyword that brings a path into scope; and the pub keyword to make items
// public. We’ll also discuss the as keyword, external packages, and the glob operator. For now, let’s
// focus on modules!

// Modules let us organize code within a crate into groups for readability and easy reuse. Modules also
// control the privacy of items, which is whether an item can be used by outside code (public) or is an
// internal implementation detail and not available for outside use (private).

// As an example, let’s write a library crate that provides the functionality of a restaurant. We’ll define
// the signatures of functions but leave their bodies empty to concentrate on the organization of the code, 
// rather than actually implement a restaurant in code.

// cargo new --lib restaurant will create a project with src/lib.rs instead of main.rs, put the
// following inside:
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// Remember src/main.rs and src/lib.rs are called crate roots
// To show Rust where to find an item in a module tree, we use a path in the same way we use a path when
// navigating a filesystem. If we want to call a function, we need to know its path. A path can take two
// forms:
//     - An absolute path starts from a crate root by using a crate name or a literal crate.
//     - A relative path starts from the current module and uses self, super, or an identifier in the current module.
// Both absolute and relative paths are followed by one or more identifiers separated by double colons (::).

// The way privacy works in Rust is that all items (functions, methods, structs, enums, modules, and constants) are
// private by default. Items in a parent module can’t use the private items inside child modules, but items in child
// modules can use the items in their ancestor modules. The reason is that child modules wrap and hide their
// implementation details, but the child modules can see the context in which they’re defined. To continue with
// the restaurant metaphor, think of the privacy rules as being like the back office of a restaurant: what goes
// on in there is private to restaurant customers, but office managers can see and do everything in the restaurant
// in which they operate.

// Example mod hosting is public an it's function add_to_waitlist is public so we could use that function
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// Using super in this context is like starting a filesystem path with  "../"
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

// Also a struct could be public while it's fields are private, so you must specify public on fields you
// want exposed as well:
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

// In contrast, if we make an enum public, all of its variants are then public. We only need the pub
// before the enum keyword
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// Bringing paths into scope with the "use" keyword. Adding use and a path in a scope is similar to
// creating a symbolic link in the filesystem
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;
// Relative path usage would look like: "use self::front_of_house::hosting;"

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// Going all the way down to the function when using use to bring into scope is unidiomatic
// Do _not_ do:
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}

// That is unless you are bringing in structs, enums, or other items then it is idiomatic
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// Again the exception to the example directly above is if you have two item you are bringing
// into scope with the same name then you can do:
use std::fmt;
use std::io;

fn function1() -> fmt::Result {}
fn function2() -> io::Result<()> {}

// OR use the "as" keyword
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {}
fn function2() -> IoResult<()> {}

// When we bring a name into scope with the use keyword, the name available in the new scope is
// private. To enable the code that calls our code to refer to that name as if it had been defined
// in that code’s scope, we can combine pub and use. This technique is called re-exporting because
// we’re bringing an item into scope but also making that item available for others to bring into
// their scope.
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// Example of external package usage:

// Cargo.toml
[dependencies]
rand = "0.5.5"

// In your code
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
}

// Note that the standard library (std) is also a crate that’s external to our package. Because the
// standard library is shipped with the Rust language, we don’t need to change Cargo.toml to include
// std. But we do need to refer to it with use to bring items from there into our package’s scope.
// For example, with HashMap we would use this line:
use std::collections::HashMap;

// Use nested paths to clean up this:
use std::io;
use std::cmp::Ordering;

// To this:
use std::{cmp::Ordering, io};

// Another example this:
use std::io;
use std::io::Write;

// To this:
use std::io::{self, Write};

// Annnddddd there's a glob operator
use std::collections::*;
