// A pointer is a general concept for a variable that contains an address in memory. This address
// refers to, or “points at,” some other data. The most common kind of pointer in Rust is a
// reference, which you learned about in Chapter 4. References are indicated by the & symbol
// and borrow the value they point to. They don’t have any special capabilities other than
// referring to data. Also, they don’t have any overhead and are the kind of pointer we use most
// often.

// Smart pointers, on the other hand, are data structures that not only act like a pointer but also
// have additional metadata and capabilities. The concept of smart pointers isn’t unique to Rust:
// smart pointers originated in C++ and exist in other languages as well. In Rust, the different
// smart pointers defined in the standard library provide functionality beyond that provided by
// references. One example that we’ll explore in this chapter is the reference counting smart pointer
// type. This pointer enables you to have multiple owners of data by keeping track of the number of
// owners and, when no owners remain, cleaning up the data.

// In Rust, which uses the concept of ownership and borrowing, an additional difference between
// references and smart pointers is that references are pointers that only borrow data; in contrast,
// in many cases, smart pointers own the data they point to.

// We’ve already encountered a few smart pointers in this book, such as String and Vec<T> in Chapter
// 8, although we didn’t call them smart pointers at the time. Both these types count as smart pointers
// because they own some memory and allow you to manipulate it. They also have metadata (such as their
// capacity) and extra capabilities or guarantees (such as with String ensuring its data will always
// be valid UTF-8).

// Smart pointers are usually implemented using structs. The characteristic that distinguishes a
// smart pointer from an ordinary struct is that smart pointers implement the Deref and Drop traits.
// The Deref trait allows an instance of the smart pointer struct to behave like a reference so you
// can write code that works with either references or smart pointers. The Drop trait allows you to
// customize the code that is run when an instance of the smart pointer goes out of scope. In this
// chapter, we’ll discuss both traits and demonstrate why they’re important to smart pointers.

// We’ll cover the most common smart pointers in the standard library:
// - Box<T> for allocating values on the heap
// - Rc<T>, a reference counting type that enables multiple ownership
// - Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

// In addition, we’ll cover the interior mutability pattern where an immutable type exposes an API
// for mutating an interior value. We’ll also discuss reference cycles: how they can leak memory and
// how to prevent them.

// Using Box<T> to Point to Data on the Heap
// The most straightforward smart pointer is a box, whose type is written Box<T>. Boxes allow you to
// store data on the heap rather than the stack. What remains on the stack is the pointer to the heap
// data. Refer to Chapter 4 to review the difference between the stack and the heap.

// Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack.
// But they don’t have many extra capabilities either. You’ll use them most often in these situations:

// - When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
// - When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
// - When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

// Below shows how to use a box to store an i32 value on the heap:
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
// We define the variable b to have the value of a Box that points to the value 5, which is allocated
// on the heap. This program will print b = 5; in this case, we can access the data in the box similar
// to how we would if this data were on the stack. Just like any owned value, when a box goes out of
// scope, as b does at the end of main, it will be deallocated. The deallocation happens for the box
// (stored on the stack) and the data it points to (stored on the heap).

// Putting a single value on the heap isn’t very useful, so you won’t use boxes by themselves in this
// way very often. Having values like a single i32 on the stack, where they’re stored by default, is
// more appropriate in the majority of situations. Let’s look at a case where boxes allow us to define
// types that we wouldn’t be allowed to if we didn’t have boxes.

// Enabling Recursive Types with Boxes
// At compile time, Rust needs to know how much space a type takes up. One type whose size can’t be known
// at compile time is a recursive type, where a value can have as part of itself another value of the same
// type. Because this nesting of values could theoretically continue infinitely, Rust doesn’t know how much
// space a value of a recursive type needs. However, boxes have a known size, so by inserting a box in a
// recursive type definition, you can have recursive types.

// Let’s explore the cons list, which is a data type common in functional programming languages, as an
// example of a recursive type. The cons list type we’ll define is straightforward except for the recursion;
// therefore, the concepts in the example we’ll work with will be useful any time you get into more complex
// situations involving recursive types.

// Because a Box<T> is a pointer, Rust always knows how much space a Box<T> needs: a pointer’s size doesn’t
// change based on the amount of data it’s pointing to. This means we can put a Box<T> inside the Cons variant
// instead of another List value directly. The Box<T> will point to the next List value that will be on the
// heap rather than inside the Cons variant. Conceptually, we still have a list, created with lists “holding”
// other lists, but this implementation is now more like placing the items next to one another rather than
// inside one another.

// We can change the definition of the List enum from before and the usage of the List from before to the
// code below, which will compile:
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}

// Treating Smart Pointers Like Regular References with the Deref Trait
// Implementing the Deref trait allows you to customize the behavior of the dereference operator, * (as opposed
// to the multiplication or glob operator). By implementing Deref in such a way that a smart pointer can be
// treated like a regular reference, you can write code that operates on references and use that code with
// smart pointers too.

// Note: there’s one big difference between the MyBox<T> type we’re about to build and the real Box<T>: our
// version will not store its data on the heap. We are focusing this example on Deref, so where the data is
// actually stored is less important than the pointer-like behavior.
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// We can rewrite the code above to use a Box<T> instead of a reference; the dereference operator will work as
// shown below:
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// Defining Our Own Smart Pointer
// Let’s build a smart pointer similar to the Box<T> type provided by the standard library to experience how
// smart pointers behave differently from references by default. Then we’ll look at how to add the ability to
// use the dereference operator.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// We define a struct named MyBox and declare a generic parameter T, because we want our type to hold values of 
// any type. The MyBox type is a tuple struct with one element of type T. The MyBox::new function takes one
// parameter of type T and returns a MyBox instance that holds the value passed in. Now the following code
// will not compile yet because we cannot dreference MyBox:
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
// Our MyBox<T> type can’t be dereferenced because we haven’t implemented that ability on our type. To enable
// dereferencing with the * operator, we implement the Deref trait.

// Treating a Type Like a Reference by Implementing the Deref Trait
// As discussed in Chapter 10, to implement a trait, we need to provide implementations for the trait’s required
// methods. The Deref trait, provided by the standard library, requires us to implement one method named deref that
// borrows self and returns a reference to the inner data. Below contains an implementation of Deref to add to the
// definition of MyBox:
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// The reason the deref method returns a reference to a value, and that the plain dereference outside the parentheses
// in *(y.deref()) is still necessary, is the ownership system. If the deref method returned the value directly
// instead of a reference to the value, the value would be moved out of self. We don’t want to take ownership of the
// inner value inside MyBox<T> in this case or in most cases where we use the dereference operator.

// Implicit Deref Coercions with Functions and Methods
// Deref coercion is a convenience that Rust performs on arguments to functions and methods. Deref coercion converts a
// reference to a type that implements Deref into a reference to a type that Deref can convert the original type into.
// Deref coercion happens automatically when we pass a reference to a particular type’s value as an argument to a function
// or method that doesn’t match the parameter type in the function or method definition. A sequence of calls to the
// deref method converts the type we provided into the type the parameter needs.
fn hello(name: &str) {
    println!("Hello {}", name);
}

// We can call the hello function with a string slice as an argument, such as hello("Rust"); for example. Deref coercion
// makes it possible to call hello with a reference to a value of type MyBox<String>
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

// Here we’re calling the hello function with the argument &m, which is a reference to a MyBox<String> value. Because we
// implemented the Deref trait on MyBox<T> in Listing 15-10, Rust can turn &MyBox<String> into &String by calling deref.
// The standard library provides an implementation of Deref on String that returns a string slice, and this is in the API
// documentation for Deref. Rust calls deref again to turn the &String into &str, which matches the hello function’s
// definition.

// If rust didn't have deref coercion, we would have to write code like:
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}

// Similar to how you use the Deref trait to override the * operator on immutable references, you can use the DerefMut trait
// to override the * operator on mutable references.

// Rust does deref coercion when it finds types and trait implementations in three cases:
// - From &T to &U when T: Deref<Target=U>
// - From &mut T to &mut U when T: DerefMut<Target=U>
// - From &mut T to &U when T: Deref<Target=U>

// // Running Code on Cleanup with the Drop Trait
// The second trait important to the smart pointer pattern is Drop, which lets you customize what happens when a value is
// about to go out of scope. You can provide an implementation for the Drop trait on any type, and the code you specify
// can be used to release resources like files or network connections. We’re introducing Drop in the context of smart
// pointers because the functionality of the Drop trait is almost always used when implementing a smart pointer. For
// example, Box<T> customizes Drop to deallocate the space on the heap that the box points to.

// In some languages, the programmer must call code to free memory or resources every time they finish using an instance
// of a smart pointer. If they forget, the system might become overloaded and crash. In Rust, you can specify that a
// particular bit of code be run whenever a value goes out of scope, and the compiler will insert this code automatically.
// As a result, you don’t need to be careful about placing cleanup code everywhere in a program that an instance of a
// particular type is finished with—you still won’t leak resources!

// Specify the code to run when a value goes out of scope by implementing the Drop trait. The Drop trait requires you to
// implement one method named drop that takes a mutable reference to self. To see when Rust calls drop, let’s implement
// drop with println! statements for now.

// Below shows a CustomSmartPointer struct whose only custom functionality is that it will print Dropping CustomSmartPointer!
// when the instance goes out of scope. This example demonstrates when Rust runs the drop function.
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("my other stuff") };
    println!("CustomSmartPointers created.");
}

// Dropping a Value Early with std::mem::drop
// Unfortunately, it’s not straightforward to disable the automatic drop functionality. Disabling drop isn’t usually necessary;
// the whole point of the Drop trait is that it’s taken care of automatically. Occasionally, however, you might want to clean up
// a value early. One example is when using smart pointers that manage locks: you might want to force the drop method that
// releases the lock to run so other code in the same scope can acquire the lock. Rust doesn’t let you call the Drop trait’s
// drop method manually; instead you have to call the std::mem::drop function provided by the standard library if you want to
// force a value to be dropped before the end of its scope.

// This error message states that we’re not allowed to explicitly call drop. The error message uses the term destructor, which is
// the general programming term for a function that cleans up an instance. A destructor is analogous to a constructor, which
// creates an instance. The drop function in Rust is one particular destructor.

// Rust doesn’t let us call drop explicitly because Rust would still automatically call drop on the value at the end of main.
// This would be a double free error because Rust would be trying to clean up the same value twice.
use std::mem::drop;

fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

// Rc<T>, the Reference Counted Smart Pointer
// In the majority of cases, ownership is clear: you know exactly which variable owns a given value. However, there are cases
// when a single value might have multiple owners. For example, in graph data structures, multiple edges might point to the
// same node, and that node is conceptually owned by all of the edges that point to it. A node shouldn’t be cleaned up unless
// it doesn’t have any edges pointing to it.

// To enable multiple ownership, Rust has a type called Rc<T>, which is an abbreviation for reference counting. The Rc<T> type
// keeps track of the number of references to a value which determines whether or not a value is still in use. If there are zero
// references to a value, the value can be cleaned up without any references becoming invalid.

// Imagine Rc<T> as a TV in a family room. When one person enters to watch TV, they turn it on. Others can come into the room and
// watch the TV. When the last person leaves the room, they turn off the TV because it’s no longer being used. If someone turns
// off the TV while others are still watching it, there would be uproar from the remaining TV watchers!

// We use the Rc<T> type when we want to allocate some data on the heap for multiple parts of our program to read and we can’t
// determine at compile time which part will finish using the data last. If we knew which part would finish last, we could just
// make that part the data’s owner, and the normal ownership rules enforced at compile time would take effect.

// Note that Rc<T> is only for use in single-threaded scenarios. When we discuss concurrency in Chapter 16, we’ll cover how to
// do reference counting in multithreaded programs.

// We could change the definition of Cons to hold references instead, but then we would have to specify lifetime parameters. By
// specifying lifetime parameters, we would be specifying that every element in the list will live at least as long as the entire
// list. The borrow checker wouldn’t let us compile let a = Cons(10, &Nil); for example, because the temporary Nil value would be
// dropped before a could take a reference to it.

// Instead, we’ll change our definition of List to use Rc<T> in place of Box<T>, as shown in Listing 15-18. Each Cons variant
// will now hold a value and an Rc<T> pointing to a List. When we create b, instead of taking ownership of a, we’ll clone the
// Rc<List> that a is holding, thereby increasing the number of references from one to two and letting a and b share ownership
// of the data in that Rc<List>. We’ll also clone a when creating c, increasing the number of references from two to three.
// Every time we call Rc::clone, the reference count to the data within the Rc<List> will increase, and the data won’t be
// cleaned up unless there are zero references to it.
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
// We could have called a.clone() rather than Rc::clone(&a), but Rust’s convention is to use Rc::clone in this case. The
// implementation of Rc::clone doesn’t make a deep copy of all the data like most types’ implementations of clone do. The
// call to Rc::clone only increments the reference count, which doesn’t take much time. Deep copies of data can take a lot
// of time. By using Rc::clone for reference counting, we can visually distinguish between the deep-copy kinds of clones
// and the kinds of clones that increase the reference count. When looking for performance problems in the code, we only
// need to consider the deep-copy clones and can disregard calls to Rc::clone.

// Cloning an Rc<T> Increases the Reference Count
// Let’s change our working example above so we can see the reference counts changing as we create and drop references to the
// Rc<List> in a. Below, we’ll change main so it has an inner scope around list c; then we can see how the reference count
// changes when c goes out of scope.
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
// This will print:
// count after creating a = 1
// count after creating b = 2
// count after creating c = 3
// count after c goes out of scope = 2

// At each point in the program where the reference count changes, we print the reference count, which we can get by calling
// the Rc::strong_count function. This function is named strong_count rather than count because the Rc<T> type also has a
// weak_count; we’ll see what weak_count is used for in the “Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>”
// section.

// We can see that the Rc<List> in a has an initial reference count of 1; then each time we call clone, the count goes up by 1.
// When c goes out of scope, the count goes down by 1. We don’t have to call a function to decrease the reference count like
// we have to call Rc::clone to increase the reference count: the implementation of the Drop trait decreases the reference
// count automatically when an Rc<T> value goes out of scope.

// What we can’t see in this example is that when b and then a go out of scope at the end of main, the count is then 0, and
// the Rc<List> is cleaned up completely at that point. Using Rc<T> allows a single value to have multiple owners, and the
// count ensures that the value remains valid as long as any of the owners still exist.

// Via immutable references, Rc<T> allows you to share data between multiple parts of your program for reading only. If Rc<T>
// allowed you to have multiple mutable references too, you might violate one of the borrowing rules discussed in Chapter 4:
// multiple mutable borrows to the same place can cause data races and inconsistencies. But being able to mutate data is very
// useful! In the next section, we’ll discuss the interior mutability pattern and the RefCell<T> type that you can use in
// conjunction with an Rc<T> to work with this immutability restriction.

// RefCell<T> and the Interior Mutability Pattern
// Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to
// that data; normally, this action is disallowed by the borrowing rules. To mutate data, the pattern uses unsafe code inside
// a data structure to bend Rust’s usual rules that govern mutation and borrowing. We haven’t yet covered unsafe code; we will
// in Chapter 19. We can use types that use the interior mutability pattern when we can ensure that the borrowing rules will
// be followed at runtime, even though the compiler can’t guarantee that. The unsafe code involved is then wrapped in a safe
// API, and the outer type is still immutable.

// Enforcing Borrowing Rules at Runtime with RefCell<T>
// Unlike Rc<T>, the RefCell<T> type represents single ownership over the data it holds. So, what makes RefCell<T> different
// from a type like Box<T>? Recall the borrowing rules you learned in Chapter 4:

// At any given time, you can have either (but not both of) one mutable reference or any number of immutable references. References
// must always be valid.

// With references and Box<T>, the borrowing rules’ invariants are enforced at compile time. With RefCell<T>, these invariants
// are enforced at runtime. With references, if you break these rules, you’ll get a compiler error. With RefCell<T>, if you
// break these rules, your program will panic and exit.

// The advantage of checking the borrowing rules at runtime instead is that certain memory-safe scenarios are then allowed,
// whereas they are disallowed by the compile-time checks. Static analysis, like the Rust compiler, is inherently conservative.
// Some properties of code are impossible to detect by analyzing the code: the most famous example is the Halting Problem, which
// is beyond the scope of this book but is an interesting topic to research.

// Because some analysis is impossible, if the Rust compiler can’t be sure the code complies with the ownership rules, it might
// reject a correct program; in this way, it’s conservative. If Rust accepted an incorrect program, users wouldn’t be able to
// trust in the guarantees Rust makes. However, if Rust rejects a correct program, the programmer will be inconvenienced, but
// nothing catastrophic can occur. The RefCell<T> type is useful when you’re sure your code follows the borrowing rules but the
// compiler is unable to understand and guarantee that.

// Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and will give you a compile-time error if you try
// using it in a multithreaded context. We’ll talk about how to get the functionality of RefCell<T> in a multithreaded program
// in Chapter 16.

// Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:

// - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.

// - Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time;
// RefCell<T> allows immutable or mutable borrows checked at runtime.

// - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when
// the RefCell<T> is immutable. Mutating the value inside an immutable value is the interior mutability pattern. Let’s look at
// a situation in which interior mutability is useful and examine how it’s possible.

// However, there are situations in which it would be useful for a value to mutate itself in its methods but appear immutable to
// other code. Code outside the value’s methods would not be able to mutate the value. Using RefCell<T> is one way to get the
// ability to have interior mutability. But RefCell<T> doesn’t get around the borrowing rules completely: the borrow checker in
// the compiler allows this interior mutability, and the borrowing rules are checked at runtime instead. If you violate the rules,
// you’ll get a panic! instead of a compiler error.

// A Use Case for Interior Mutability: Mock Objects
// A test double is the general programming concept for a type used in place of another type during testing. Mock objects are specific
// types of test doubles that record what happens during a test so you can assert that the correct actions took place.
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
             self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

// Keeping Track of Borrows at Runtime with RefCell<T>
// When creating immutable and mutable references, we use the & and &mut syntax, respectively. With RefCell<T>, we use
// the borrow and borrow_mut methods, which are part of the safe API that belongs to RefCell<T>. The borrow method
// returns the smart pointer type Ref<T>, and borrow_mut returns the smart pointer type RefMut<T>. Both types implement
// Deref, so we can treat them like regular references.

// The RefCell<T> keeps track of how many Ref<T> and RefMut<T> smart pointers are currently active. Every time we call
// borrow, the RefCell<T> increases its count of how many immutable borrows are active. When a Ref<T> value goes out of
// scope, the count of immutable borrows goes down by one. Just like the compile-time borrowing rules, RefCell<T> lets us
// have many immutable borrows or one mutable borrow at any point in time.

// If we try to violate these rules, rather than getting a compiler error as we would with references, the implementation
// of RefCell<T> will panic at runtime. Listing 15-23 shows a modification of the implementation of send in Listing 15-22.
// We’re deliberately trying to create two mutable borrows active for the same scope to illustrate that RefCell<T> prevents
// us from doing this at runtime. This code panics!
impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        let mut one_borrow = self.sent_messages.borrow_mut();
        let mut two_borrow = self.sent_messages.borrow_mut();

        one_borrow.push(String::from(message));
        two_borrow.push(String::from(message));
    }
}

// Catching borrowing errors at runtime rather than compile time means that you would find a mistake in your code later in the
// development process and possibly not until your code was deployed to production. Also, your code would incur a small runtime
// performance penalty as a result of keeping track of the borrows at runtime rather than compile time. However, using RefCell<T>
// makes it possible to write a mock object that can modify itself to keep track of the messages it has seen while you’re using
// it in a context where only immutable values are allowed. You can use RefCell<T> despite its trade-offs to get more
// functionality than regular references provide.

// Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>
// A common way to use RefCell<T> is in combination with Rc<T>. Recall that Rc<T> lets you have multiple owners of some data, but
// it only gives immutable access to that data. If you have an Rc<T> that holds a RefCell<T>, you can get a value that can have
// multiple owners and that you can mutate!

// For example, recall the cons list example in Listing 15-18 where we used Rc<T> to allow multiple lists to share ownership of
// another list. Because Rc<T> holds only immutable values, we can’t change any of the values in the list once we’ve created
// them. Let’s add in RefCell<T> to gain the ability to change the values in the lists. Listing 15-24 shows that by using a
// RefCell<T> in the Cons definition, we can modify the value stored in all the lists:
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// We create a value that is an instance of Rc<RefCell<i32>> and store it in a variable named value so we can access it directly
// later. Then we create a List in a with a Cons variant that holds value. We need to clone value so both a and value have ownership
// of the inner 5 value rather than transferring ownership from value to a or having a borrow from value.

// We wrap the list a in an Rc<T> so when we create lists b and c, they can both refer to a, which is what we did in Listing 15-18.

// After we’ve created the lists in a, b, and c, we add 10 to the value in value. We do this by calling borrow_mut on value, which
// uses the automatic dereferencing feature we discussed in Chapter 5 (see the section “Where’s the -> Operator?”) to dereference
// the Rc<T> to the inner RefCell<T> value. The borrow_mut method returns a RefMut<T> smart pointer, and we use the dereference
// operator on it and change the inner value.

// When we print a, b, and c, we can see that they all have the modified value of 15 rather than 5:
// a after = Cons(RefCell { value: 15 }, Nil)
// b after = Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
// c after = Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))