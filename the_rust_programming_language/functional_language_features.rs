// Creating an Abstraction of Behavior with Closures
// A function to stand in for a hypothetical calculation that takes about 2 seconds to run:
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn simulated_expensive_calculation(intensity: i32) -> i32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: i32, random_number: i32) {
    if intensity > 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

// Refactoring Using Functions
// We could restructure the workout program in many ways. First, we’ll try extracting the duplicated call
// to the simulated_expensive_calculation function into a variable:
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}

// Lets turn that first call into a closure:
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |intensity| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

// The rust compiler can infer types, etc for a closure. If you want to be more specific however, you can.
let expensive_closure = |intensity: i32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
};

// Once types are inferred however, they are locked in. We can't call a closure with different types. The
// first time we call example_closure with the String value, the compiler infers the type of x and the
// return type of the closure to be String. Those types are then locked in to the closure in example_closure,
// and we get a type error if we try to use a different type with the same closure. This code will fail:
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);

// Storing Closures Using Generic Parameters and the Fn Traits
// Fortunately, another solution is available to us. We can create a struct that will hold the closure and the
// resulting value of calling the closure. The struct will execute the closure only if we need the resulting
// value, and it will cache the resulting value so the rest of our code doesn’t have to be responsible for saving
// and reusing the result. You may know this pattern as memoization or lazy evaluation.

// To make a struct that holds a closure, we need to specify the type of the closure, because a struct definition
// needs to know the types of each of its fields. Each closure instance has its own unique anonymous type: that is,
// even if two closures have the same signature, their types are still considered different. To define structs,
// enums, or function parameters that use closures, we use generics and trait bounds.

// The Fn traits are provided by the standard library. All closures implement at least one of the traits: Fn, FnMut,
// or FnOnce. We’ll discuss the difference between these traits in the “Capturing the Environment with Closures”
// section; in this example, we can use the Fn trait.

// We add types to the Fn trait bound to represent the types of the parameters and return values the closures must have
// to match this trait bound. In this case, our closure has a parameter of type u32 and returns a u32, so the trait
// bound we specify is Fn(u32) -> u32.
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// We want Cacher to manage the struct fields’ values rather than letting the calling code potentially change the values
// in these fields directly, so these fields are private.

// The Cacher::new function takes a generic parameter T, which we’ve defined as having the same trait bound as the Cacher
// struct. Then Cacher::new returns a Cacher instance that holds the closure specified in the calculation field and a
// None value in the value field, because we haven’t executed the closure yet.

// When the calling code needs the result of evaluating the closure, instead of calling the closure directly, it will
// call the value method. This method checks whether we already have a resulting value in self.value in a Some; if we
// do, it returns the value within the Some without executing the closure again.

// If self.value is None, the code calls the closure stored in self.calculation, saves the result in self.value for future
// use, and returns the value as well.
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
    });

    if intensity > 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_num == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

// Limitations of the Cacher Implementation
// Caching values is a generally useful behavior that we might want to use in other parts of our code with different closures.
// However, there are two problems with the current implementation of Cacher that would make reusing it in different contexts
// difficult.

// The first problem is that a Cacher instance assumes it will always get the same value for the parameter arg to the value
// method. That is, this test of Cacher will fail:
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
// The problem is that the first time we called c.value with 1, the Cacher instance saved Some(1) in self.value. Thereafter, no
// matter what we pass in to the value method, it will always return 1.

// Try modifying Cacher to hold a hash map rather than a single value. The keys of the hash map will be the arg values that are passed
// in, and the values of the hash map will be the result of calling the closure on that key. Instead of looking at whether self.value
// directly has a Some or a None value, the value function will look up the arg in the hash map and return the value if it’s present.
// If it’s not present, the Cacher will call the closure and save the resulting value in the hash map associated with its arg value.

// The second problem with the current Cacher implementation is that it only accepts closures that take one parameter of type u32 and
// return a u32. We might want to cache the results of closures that take a string slice and return usize values, for example. To fix
// this issue, try introducing more generic parameters to increase the flexibility of the Cacher functionality.

// Capturing the Environment with Closures
// In the workout generator example, we only used closures as inline anonymous functions. However, closures have an additional
// capability that functions don’t have: they can capture their environment and access variables from the scope in which they’re
// defined.
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}

// Here, even though x is not one of the parameters of equal_to_x, the equal_to_x closure is allowed to use the x variable that’s
// defined in the same scope that equal_to_x is defined in. We can’t do the same with functions; if we try with the following
// example, our code won’t compile:
fn main() {
    let x = 4;

    fn equal_to_x(z: i32) -> bool { z == x }

    let y = 4;

    assert!(equal_to_x(y));
}
// When a closure captures a value from its environment, it uses memory to store the values for use in the closure body. This
// use of memory is overhead that we don’t want to pay in more common cases where we want to execute code that doesn’t capture
// its environment. Because functions are never allowed to capture their environment, defining and using functions will never
// incur this overhead.

// Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a
// parameter: taking ownership, borrowing mutably, and borrowing immutably. These are encoded in the three Fn traits as follows:

// 1. FnOnce consumes the variables it captures from its enclosing scope, known as the closure’s environment. To consume the captured
// variables, the closure must take ownership of these variables and move them into the closure when it is defined. The Once part of
// the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only
// once.

// 2. FnMut can change the environment because it mutably borrows values.

// 3. Fn borrows values from the environment immutably.

// When you create a closure, Rust infers which trait to use based on how the closure uses the values from the environment. All
// closures implement FnOnce because they can all be called at least once. Closures that don’t move the captured variables also
// implement FnMut, and closures that don’t need mutable access to the captured variables also implement Fn.

// If you want to force the closure to take ownership of the values it uses in the environment, you can use the move keyword before
// the parameter list. This technique is mostly useful when passing a closure to a new thread to move the data so it’s owned by the
// new thread.

// We’ll have more examples of move closures in Chapter 16 when we talk about concurrency. For now, here’s the code with the move
// keyword added to the closure definition and using vectors instead of integers, because integers can be copied rather than moved;
// note that this code will not yet compile.
fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}", x); // <- this will break compilation because x is now moved into the closure

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

// Processing a Series of Items with Iterators
// The iterator pattern allows you to perform some task on a sequence of items in turn. An iterator is responsible for the
// logic of iterating over each item and determining when the sequence has finished. When you use iterators, you don’t have
// to reimplement that logic yourself.

// In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up. For
// example, the code in Listing 13-13 creates an iterator over the items in the vector v1 by calling the iter method defined
// on Vec<T>. This code by itself doesn’t do anything useful.
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {}", val);
}

// The Iterator Trait and the next Method
// All iterators implement a trait named Iterator that is defined in the standard library. The definition of the trait looks
// like this:
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}

// Notice this definition uses some new syntax: type Item and Self::Item, which are defining an associated type with this trait.
// We’ll talk about associated types in depth in Chapter 19. For now, all you need to know is that this code says implementing
// the Iterator trait requires that you also define an Item type, and this Item type is used in the return type of the next
// method. In other words, the Item type will be the type returned from the iterator.

// We can call the next method on iterators directly; Listing 13-15 demonstrates what values are returned from repeated calls to
// next on the iterator created from the vector.
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

// Note that we needed to make v1_iter mutable: calling the next method on an iterator changes internal state that the iterator
// uses to keep track of where it is in the sequence. In other words, this code consumes, or uses up, the iterator. Each call to
// next eats up an item from the iterator. We didn’t need to make v1_iter mutable when we used a for loop because the loop took
// ownership of v1_iter and made it mutable behind the scenes.

// Also note that the values we get from the calls to next are immutable references to the values in the vector. The iter method
// produces an iterator over immutable references. If we want to create an iterator that takes ownership of v1 and returns owned
// values, we can call into_iter instead of iter. Similarly, if we want to iterate over mutable references, we can call iter_mut
// instead of iter.

// Methods that call next are called consuming adaptors, because calling them uses up the iterator. One example is the sum method,
// which takes ownership of the iterator and iterates through the items by repeatedly calling next, thus consuming the iterator.
// As it iterates through, it adds each item to a running total and returns the total when iteration is complete. Below has a test
// illustrating a use of the sum method:
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the iterator we call it on.
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

// Other methods defined on the Iterator trait, known as iterator adaptors, allow you to change iterators into different
// kinds of iterators. You can chain multiple calls to iterator adaptors to perform complex actions in a readable way.
// But because all iterators are lazy, you have to call one of the consuming adaptor methods to get results from calls
// to iterator adaptors.

// Listing 13-17 shows an example of calling the iterator adaptor method map, which takes a closure to call on each item
// to produce a new iterator. The closure here creates a new iterator in which each item from the vector has been incremented
// by 1. However, this code produces a warning:
let v1: Vec<i32> = vec![1, 2, 3];
v2.iter().map(|x| x + 1);
// This gives us: warning: unused `std::iter::Map` which must be used: iterator adaptors are lazy and do nothing unless consumed

// So we should do this instead:
let v1: Vec<i32> = vec![1, 2, 3];
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
assert_eq!(v2, vec![2, 3, 4]);

// Using filter with a closure
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}

// Creating Our Own Iterators with the Iterator Trait
// We’ll implement the Iterator trait for our Counter type by defining the body of the next method to specify what
// we want to happen when this iterator is used
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

// We set the associated Item type for our iterator to u32, meaning the iterator will return u32 values. Again,
// don’t worry about associated types yet, we’ll cover them in Chapter 19.

// We want our iterator to add 1 to the current state, so we initialized count to 0 so it would return 1 first. If
// the value of count is less than 6, next will return the current value wrapped in Some, but if count is 6 or
// higher, our iterator will return None

// Using Our Counter Iterator’s next Method
#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

// Using Other Iterator Trait Methods
// We implemented the Iterator trait by defining the next method, so we can now use any Iterator trait method’s default
// implementations as defined in the standard library, because they all use the next method’s functionality.

// For example, if for some reason we wanted to take the values produced by an instance of Counter, pair them with
// values produced by another Counter instance after skipping the first value, multiply each pair together, keep
// only those results that are divisible by 3, and add all the resulting values together, we could do so:
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
}

// All of these method calls are possible because we specified how the next method works, and the standard library
// provides default implementations for other methods that call next.

// Now we went over to minigrep and refactored with iterators. The question becomes which is faster?
// To determine whether to use loops or iterators, you need to know which version of our search functions
// is faster: the version with an explicit for loop or the version with iterators.

// Benchmark shows the iterators were actually faster! Iterators are one of Rust’s zero-cost abstractions,
// by which we mean using the abstraction imposes no additional runtime overhead. This is analogous to how
// Bjarne Stroustrup, the original designer and implementor of C++, defines zero-overhead in “Foundations of
// C++” (2012):

// In general, C++ implementations obey the zero-overhead principle: What you don’t use, you don’t pay for. And further:
// What you do use, you couldn’t hand code any better.

let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients
        .iter()
        .zip(&buffer[i - 12..i])
        .map(|(&c, &s)| c * s as i64)
        .sum::<i64>() >> qlp_shift;

    let delta = buffer[i];

    buffer[i] = prediction as i32 + delta;
}

// To calculate the value of prediction, this code iterates through each of the 12 values in coefficients and uses the
// zip method to pair the coefficient values with the previous 12 values in buffer. Then, for each pair, we multiply the
// values together, sum all the results, and shift the bits in the sum qlp_shift bits to the right.

// Calculations in applications like audio decoders often prioritize performance most highly. Here, we’re creating an
// iterator, using two adaptors, and then consuming the value. What assembly code would this Rust code compile to?
// Well, as of this writing, it compiles down to the same assembly you’d write by hand. There’s no loop at all
// corresponding to the iteration over the values in coefficients: Rust knows that there are 12 iterations, so it
// “unrolls” the loop. Unrolling is an optimization that removes the overhead of the loop controlling code and instead
// generates repetitive code for each iteration of the loop.

// All of the coefficients get stored in registers, which means accessing the values is very fast. There are no bounds
// checks on the array access at runtime. All these optimizations that Rust is able to apply make the resulting code
// extremely efficient. Now that you know this, you can use iterators and closures without fear! They make code seem
// like it’s higher level but don’t impose a runtime performance penalty for doing so.

