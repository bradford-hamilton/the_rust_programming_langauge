// Cargo has two main profiles: the dev profile Cargo uses when you run cargo build and the
// release profile Cargo uses when you run cargo build --release. The dev profile is defined
// with good defaults for development, and the release profile has good defaults for release
// builds.

// Cargo has default settings for each of the profiles that apply when there aren’t any [profile.*]
// sections in the project’s Cargo.toml file. By adding [profile.*] sections for any profile you
// want to customize, you can override any subset of the default settings. For example, here are
// the default values for the opt-level setting for the dev and release profiles:
// Cargo.toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3

// The opt-level setting controls the number of optimizations Rust will apply to your code, with
// a range of 0 to 3. Applying more optimizations extends compiling time, so if you’re in development
// and compiling your code often, you’ll want faster compiling even if the resulting code runs slower.
// That is the reason the default opt-level for dev is 0. When you’re ready to release your code,
// it’s best to spend more time compiling. You’ll only compile in release mode once, but you’ll run
// the compiled program many times, so release mode trades longer compile time for code that runs
// faster. That is why the default opt-level for the release profile is 3.

// You can override any default setting by adding a different value for it in Cargo.toml. For example,
// if we want to use optimization level 1 in the development profile, we can add these two lines to
// our project’s Cargo.toml file:
[profile.dev]
opt-level = 1

// Publishing a Crate to Crates.io
// Making Useful Documentation Comments

// Documentation comments use three slashes instead of two and supports Markdown notation for formatting
// the text. Place documentation comments just before the item they’re documenting.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// This command runs the rustdoc tool distributed with Rust and puts the generated HTML documentation in the target/doc directory.
$ cargo doc

// And to open them
$ cargo doc --open

// Documentation Comments as Tests
// Adding example code blocks in your documentation comments can help demonstrate how to use your library, and doing so
// has an additional bonus: running cargo test will run the code examples in your documentation as tests! Nothing is
// better than documentation with examples. But nothing is worse than examples that don’t work because the code has
// changed since the documentation was written.

// Now if we change either the function or the example so the assert_eq! in the example panics and run cargo test
// again, we’ll see that the doc tests catch that the example and the code are out of sync with each other!

// Another style of doc comment, //!, adds documentation to the item that contains the comments rather than adding
// documentation to the items following the comments. We typically use these doc comments inside the crate root file
// (src/lib.rs by convention) or inside a module to document the crate or the module as a whole.

// For example, if we want to add documentation that describes the purpose of the my_crate crate that contains the add_one
// function, we can add documentation comments that start with //! to the beginning of the src/lib.rs file, as shown:

// src/lib.rs

//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--

// Exporting a Convenient Public API with pub use
// The structure of your public API is a major consideration when publishing a crate. People who use your crate are less
// familiar with the structure than you are and might have difficulty finding the pieces they want to use if your crate
// has a large module hierarchy.

// The good news is that if the structure isn’t convenient for others to use from another library, you don’t have to rearrange
// your internal organization: instead, you can re-export items to make a public structure that’s different from your private
// structure by using pub use. Re-exporting takes a public item in one location and makes it public in another location, as 
// f it were defined in the other location instead.

// For example, say we made a library named art for modeling artistic concepts. Within this library are two modules: a kinds
// module containing two enums named PrimaryColor and SecondaryColor and a utils module containing a function named mix:

//! # Art
//!
//! A library for modeling artistic concepts.
pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
    }
}

// Note that the PrimaryColor and SecondaryColor types aren’t listed on the front page, nor is the mix function.
// We have to click kinds and utils to see them. Another crate that depends on this library would need use statements
// that bring the items from art into scope, specifying the module structure that’s currently defined. Below shows
// an example of a crate that uses the PrimaryColor and mix items from the art crate:
use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;

    mix(red, yellow);
}

// To remove the internal organization from the public API, we can modify the art crate code from above to add pub use
// statements to re-export the items at the top level.

//! # Art
//!
//! A library for modeling artistic concepts.
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    // --snip--
}

pub mod utils {
    // --snip--
}

// Now the example main fn above can look like:
use art::PrimaryColor;
use art::mix;

fn main() {
    // --snip--
}

// Publishing to crates.io
// https://crates.io/me for your API key, then login with it
$ cargo login 123_some_api_key_456
// This command will inform Cargo of your API token and store it locally in ~/.cargo/credentials

// Adding Metadata to a New Crate
// Now that you have an account, let’s say you have a crate you want to publish. Before publishing, you’ll need to add
// some metadata to your crate by adding it to the [package] section of the crate’s Cargo.toml file.
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

// Be careful when publishing a crate because a publish is permanent. The version can never be overwritten, and the code
// cannot be deleted. One major goal of crates.io is to act as a permanent archive of code so that builds of all projects
// that depend on crates from crates.io will continue to work. Allowing version deletions would make fulfilling that goal
// impossible. However, there is no limit to the number of crate versions you can publish.
$ cargo publish

// Publishing a New Version of an Existing Crate
// When you’ve made changes to your crate and are ready to release a new version, you change the version value specified in
// your Cargo.toml file and republish. Use the Semantic Versioning rules to decide what an appropriate next version number
// is based on the kinds of changes you’ve made. Then run cargo publish to upload the new version.

// Removing Versions from Crates.io with cargo yank
// Although you can’t remove previous versions of a crate, you can prevent any future projects from adding them as a new
// dependency. This is useful when a crate version is broken for one reason or another. In such situations, Cargo supports
// yanking a crate version.

// Yanking a version prevents new projects from starting to depend on that version while allowing all existing projects that
// depend on it to continue to download and depend on that version. Essentially, a yank means that all projects with a
// Cargo.lock will not break, and any future Cargo.lock files generated will not use the yanked version.

// To yank a version of a crate, run cargo yank and specify which version you want to yank:
$ cargo yank --vers 1.0.1

// By adding --undo to the command, you can also undo a yank and allow projects to start depending on a version again:
$ cargo yank --vers 1.0.1 --undo

// A yank does not delete any code. For example, the yank feature is not intended for deleting accidentally uploaded secrets.
// If that happens, you must reset those secrets immediately.