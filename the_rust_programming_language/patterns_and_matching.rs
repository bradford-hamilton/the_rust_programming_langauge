// Patterns are a special syntax in Rust for matching against the structure of types, both complex
// and simple. Using patterns in conjunction with match expressions and other constructs gives you
// more control over a program’s control flow. A pattern consists of some combination of the
// following:

// - Literals
// - Destructured arrays, enums, structs, or tuples
// - Variables
// - Wildcards
// - Placeholders

// All the Places Patterns Can Be Used
// match Arms
// As discussed in Chapter 6, we use patterns in the arms of match expressions. Formally, match
// expressions are defined as the keyword match, a value to match on, and one or more match arms
// that consist of a pattern and an expression to run if the value matches that arm’s pattern,
// like this:
// match VALUE {
//     PATTERN => EXPRESSION,
//     PATTERN => EXPRESSION,
//     PATTERN => EXPRESSION,
// }

// One requirement for match expressions is that they need to be exhaustive in the sense that all
// possibilities for the value in the match expression must be accounted for. One way to ensure
// you’ve covered every possibility is to have a catchall pattern for the last arm: for example,
// a variable name matching any value can never fail and thus covers every remaining case.

// A particular pattern _ will match anything, but it never binds to a variable, so it’s often
// used in the last match arm. The _ pattern can be useful when you want to ignore any value not
// specified, for example. We’ll cover the _ pattern in more detail in the “Ignoring Values in
// a Pattern” section later in this chapter.

// Conditional if let Expressions
// In Chapter 6 we discussed how to use if let expressions mainly as a shorter way to write the
// equivalent of a match that only matches one case. Optionally, if let can have a corresponding
// else containing code to run if the pattern in the if let doesn’t match.

// Listing 18-1 shows that it’s also possible to mix and match if let, else if, and else if let
// expressions. Doing so gives us more flexibility than a match expression in which we can express
// only one value to compare with the patterns. Also, the conditions in a series of if let, else if,
// else if let arms aren’t required to relate to each other.
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

// while let Conditional Loops
// Similar in construction to if let, the while let conditional loop allows a while loop to run for
// as long as a pattern continues to match. The example in Listing 18-2 shows a while let loop that
// uses a vector as a stack and prints the values in the vector in the opposite order in which they
// were pushed.
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
// This example prints 3, 2, and then 1. The pop method takes the last element out of the vector and
// returns Some(value). If the vector is empty, pop returns None. The while loop continues running
// the code in its block as long as pop returns Some. When pop returns None, the loop stops. We can
// use while let to pop every element off our stack.