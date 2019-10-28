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

// Listing 18-3 demonstrates how to use a pattern in a for loop to destructure, or break apart, a
// tuple as part of the for loop.
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
// Prints:
// a is at index 0
// b is at index 1
// c is at index 2

// let Statements
// Throughout this book, we’ve used let like this hundreds of times, and although you might not have
// realized it, you were using patterns! More formally, a let statement looks like this:
let x = 5;
// let PATTERN = EXPRESSION;
// In statements like let x = 5; with a variable name in the PATTERN slot, the variable name is just a
// particularly simple form of a pattern. Rust compares the expression against the pattern and assigns
// any names it finds. So in the let x = 5; example, x is a pattern that means “bind what matches here
// to the variable x.” Because the name x is the whole pattern, this pattern effectively means “bind
// everything to the variable x, whatever the value is.”

// To see the pattern matching aspect of let more clearly, consider Listing 18-4, which uses a pattern
// with let to destructure a tuple.
let (x, y, z) = (1, 2, 3);
// Here, we match a tuple against a pattern. Rust compares the value (1, 2, 3) to the pattern (x, y, z)
// and sees that the value matches the pattern, so Rust binds 1 to x, 2 to y, and 3 to z. You can think
// of this tuple pattern as nesting three individual variable patterns inside it.

// Function parameters can also be patterns. The code in Listing 18-6, which declares a function named foo
// that takes one parameter named x of type i32, should by now look familiar.
fn foo(x: i32) {
    // code goes here
}

// The x part is a pattern! As we did with let, we could match a tuple in a function’s arguments to the
// pattern. Listing 18-7 splits the values in a tuple as we pass it to a function.
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}

// Patterns come in two forms: refutable and irrefutable. Patterns that will match for any possible value
// passed are irrefutable. An example would be x in the statement let x = 5; because x matches anything
// and therefore cannot fail to match. Patterns that can fail to match for some possible value are refutable.
// An example would be Some(x) in the expression if let Some(x) = a_value because if the value in the a_value
// variable is None rather than Some, the Some(x) pattern will not match.

// Function parameters, let statements, and for loops can only accept irrefutable patterns, because the
// program cannot do anything meaningful when values don’t match. The if let and while let expressions
// only accept refutable patterns, because by definition they’re intended to handle possible failure:
// the functionality of a conditional is in its ability to perform differently depending on success or
// failure.

// In general, you shouldn’t have to worry about the distinction between refutable and irrefutable patterns;
// however, you do need to be familiar with the concept of refutability so you can respond when you see it
// in an error message. In those cases, you’ll need to change either the pattern or the construct you’re
// using the pattern with, depending on the intended behavior of the code.

// Let’s look at an example of what happens when we try to use a refutable pattern where Rust requires an
// irrefutable pattern and vice versa. Listing 18-8 shows a let statement, but for the pattern we’ve
// specified Some(x), a refutable pattern. As you might expect, this code will not compile.
let Some(x) = some_option_value;
// If some_option_value was a None value, it would fail to match the pattern Some(x), meaning the pattern is
// refutable. However, the let statement can only accept an irrefutable pattern because there is nothing valid
// the code can do with a None value. At compile time, Rust will complain that we’ve tried to use a refutable
// pattern where an irrefutable pattern is required:

// To fix the problem where we have a refutable pattern where an irrefutable pattern is needed, we can change
// the code that uses the pattern: instead of using let, we can use if let. Then if the pattern doesn’t match,
// the code will just skip the code in the curly brackets, giving it a way to continue validly. Listing 18-9
// shows how to fix the code in Listing 18-8.
if let Some(x) = some_option_value {
    println!("{}", x);
}

// We’ve given the code an out! This code is perfectly valid, although it means we cannot use an irrefutable
// pattern without receiving an error. If we give if let a pattern that will always match, such as x, as shown
// in Listing 18-10, it will not compile.

// Pattern Syntax
// As you saw in Chapter 6, you can match patterns against literals directly. The following code gives some examples:
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}

// Matching Named Variables
// Named variables are irrefutable patterns that match any value, and we’ve used them many times in the book.
// However, there is a complication when you use named variables in match expressions. Because match starts a
// new scope, variables declared as part of a pattern inside the match expression will shadow those with the
// same name outside the match construct, as is the case with all variables. In Listing 18-11, we declare a
// variable named x with the value Some(5) and a variable y with the value 10. We then create a match expression
// on the value x. Look at the patterns in the match arms and println! at the end, and try to figure out what
// the code will print before running this code or reading further.
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

// Let’s walk through what happens when the match expression runs. The pattern in the first match arm doesn’t
// match the defined value of x, so the code continues.

// The pattern in the second match arm introduces a new variable named y that will match any value inside a Some
// value. Because we’re in a new scope inside the match expression, this is a new y variable, not the y we declared
// at the beginning with the value 10. This new y binding will match any value inside a Some, which is what we have
// in x. Therefore, this new y binds to the inner value of the Some in x. That value is 5, so the expression for
// that arm executes and prints Matched, y = 5.

// If x had been a None value instead of Some(5), the patterns in the first two arms wouldn’t have matched, so the
// value would have matched to the underscore. We didn’t introduce the x variable in the pattern of the underscore
// arm, so the x in the expression is still the outer x that hasn’t been shadowed. In this hypothetical case, the
// match would print Default case, x = None.

// When the match expression is done, its scope ends, and so does the scope of the inner y. The last println!
// produces at the end: x = Some(5), y = 10.

// To create a match expression that compares the values of the outer x and y, rather than introducing a shadowed
// variable, we would need to use a match guard conditional instead. We’ll talk about match guards later in the
// “Extra Conditionals with Match Guards” section.

// In match expressions, you can match multiple patterns using the | syntax, which means or. For example, the
// following code matches the value of x against the match arms, the first of which has an or option, meaning
// if the value of x matches either of the values in that arm, that arm’s code will run:
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}

// Matching Ranges of Values with ...
// The ... syntax allows us to match to an inclusive range of values. In the following code, when a pattern matches
// any of the values within the range, that arm will execute:
let x = 5;

match x {
    1...5 => println!("one through five"),
    _ => println("something else"),
}

// Here is an example using ranges of char values:
let x = 'c';

match x {
    'a'...'j' => println!("early ASCII letter"),
    'k'...'z' => println!("late ASCII letter"),
    _ => println!("something else"),
}

// Destructuring to Break Apart Values
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;

    assert_eq!(0, a);
    assert_eq!(7, b);
}

// Because having variable names match the fields is common and because writing let Point { x: x, y: y } = p; contains
// a lot of duplication, there is a shorthand for patterns that match struct fields: you only need to list the name of
// the struct field, and the variables created from the pattern will have the same names. Listing 18-13 shows code that
// behaves in the same way as the code in Listing 18-12, but the variables created in the let pattern are x and y
// instead of a and b.
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;

    assert_eq!(0, x);
    assert_eq!(7, y);
}

// Or even
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}


// Destructuring Enums
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        },
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        },
    }
}
// This code will print Change the color to red 0, green 160, and blue 255. Try changing the value of
// msg to see the code from the other arms run.

// For enum variants without any data, like Message::Quit, we can’t destructure the value any further. We
// can only match on the literal Message::Quit value, and no variables are in that pattern.

// For struct-like enum variants, such as Message::Move, we can use a pattern similar to the pattern we
// specify to match structs. After the variant name, we place curly brackets and then list the fields with
// variables so we break apart the pieces to use in the code for this arm. Here we use the shorthand form
// as we did in Listing 18-13.

// For tuple-like enum variants, like Message::Write that holds a tuple with one element and
// Message::ChangeColor that holds a tuple with three elements, the pattern is similar to the
// pattern we specify to match tuples. The number of variables in the pattern must match the number
// of elements in the variant we’re matching.

// Destructuring Nested Structs and Enums
// Until now, all our examples have been matching structs or enums that were one level deep. Matching can work
// on nested items too!

// For example, we can refactor the code in Listing 18-15 to support RGB and HSV colors in the ChangeColor
// message, as shown in Listing 18-16.

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        },
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {}, saturation {}, and value {}", h, s, v)
        }
        _ => ()
    }
}

// Destructuring Structs and Tuples
let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });

// Ignoring an Entire Value with _
// We’ve used the underscore (_) as a wildcard pattern that will match any value but not bind to the value.
// Although the underscore _ pattern is especially useful as the last arm in a match expression, we can use
// it in any pattern, including function parameters, as shown in Listing 18-17.
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}

// Ignoring Parts of a Value with a Nested _
// We can also use _ inside another pattern to ignore just part of a value, for example, when we want to test
// for only part of a value but have no use for the other parts in the corresponding code we want to run.
// Listing 18-18 shows code responsible for managing a setting’s value. The business requirements are that
// the user should not be allowed to overwrite an existing customization of a setting but can unset the
// setting and give it a value if it is currently unset.
let mut setting_value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
        println!("Can't overwrite an existing customized value");
    }
    _ => {
        setting_value = new_setting_value;
    }
}

println!("setting is {:?}", setting_value);
// This code will print Can't overwrite an existing customized value and then setting is Some(5). In the first
// match arm, we don’t need to match on or use the values inside either Some variant, but we do need to test for
// the case when setting_value and new_setting_value are the Some variant. In that case, we print why we’re not
// changing setting_value, and it doesn’t get changed.

// We can also use underscores in multiple places within one pattern to ignore particular values. Below shows an
// example of ignoring the second and fourth values in a tuple of five items.
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth)
    },
}
// This code will print Some numbers: 2, 8, 32, and the values 4 and 16 will be ignored.

// Ignoring Remaining Parts of a Value with ..
// With values that have many parts, we can use the .. syntax to use only a few parts and ignore the rest, avoiding
// the need to list underscores for each ignored value. The .. pattern ignores any parts of a value that we haven’t
// explicitly matched in the rest of the pattern. In Listing 18-23, we have a Point struct that holds a coordinate
// in three-dimensional space. In the match expression, we want to operate only on the x coordinate and ignore the
// values in the y and z fields.
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}

// We list the x value and then just include the .. pattern. This is quicker than having to list y: _ and z: _,
// particularly when we’re working with structs that have lots of fields in situations where only one or two
// fields are relevant. The syntax .. will expand to as many values as it needs to be. Below shows how to use ..
// with a tuple.
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}

// In this code, the first and last value are matched with first and last. The .. will match and ignore everything
// in the middle. However, using .. must be unambiguous. If it is unclear which values are intended for matching
// and which should be ignored, Rust will give us an error. Listing 18-25 shows an example of using .. ambiguously,
// so it will not compile. This code does not compile!
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
    }
}

// Extra Conditionals with Match Guards
// A match guard is an additional if condition specified after the pattern in a match arm that must also match, along
// with the pattern matching, for that arm to be chosen. Match guards are useful for expressing more complex ideas
// than a pattern alone allows.
let num = Some(4);

match num {
    Some(x) => if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
// This example will print less than five: 4. When num is compared to the pattern in the first arm, it matches, because
// Some(4) matches Some(x). Then the match guard checks whether the value in x is less than 5, and because it is, the
// first arm is selected.

// Using a match guard to test for equality with an outer variable
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}
// This code will now print Default case, x = Some(5). The pattern in the second match arm doesn’t introduce a new variable
// y that would shadow the outer y, meaning we can use the outer y in the match guard. Instead of specifying the pattern as
// Some(y), which would have shadowed the outer y, we specify Some(n). This creates a new variable n that doesn’t shadow
// anything because there is no n variable outside the match.

// The match guard if n == y is not a pattern and therefore doesn’t introduce new variables. This y is the outer y rather than
// a new shadowed y, and we can look for a value that has the same value as the outer y by comparing n to y.

// You can also use the or operator | in a match guard to specify multiple patterns; the match guard condition will apply to
// all the patterns. Listing 18-28 shows the precedence of combining a match guard with a pattern that uses |. The important
// part of this example is that the if y match guard applies to 4, 5, and 6, even though it might look like if y only applies
// to 6.
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
}

// @ Bindings
// The at operator (@) lets us create a variable that holds a value at the same time we’re testing that value to see whether
// it matches a pattern. Listing 18-29 shows an example where we want to test that a Message::Hello id field is within the
// range 3...7. But we also want to bind the value to the variable id_variable so we can use it in the code associated with
// the arm. We could name this variable id, the same as the field, but for this example we’ll use a different name.
enum Message {
    Hello { id: 2 },
}

let msg = Message::Hello { id: 5 }

match msg {
    Message::Hello { id: id_variable @ 3..7 } => {
        println!("Found an id in range: {}", id_variable)
    },
    Message::Hello { id: 10..12 } => {
        println!("Found an id in another range")
    },
    Message::Hello { id } => {
        println!("Found some other id: {}", id)
    },
}
// This example will print Found an id in range: 5. By specifying id_variable @ before the range 3...7, we’re capturing whatever
// value matched the range while also testing that the value matched the range pattern.