// Characteristics of Object-Oriented Languages
// There is no consensus in the programming community about what features a language must have to be
// considered object oriented. Rust is influenced by many programming paradigms, including OOP; for
// example, we explored the features that came from functional programming in Chapter 13. Arguably,
// OOP languages share certain common characteristics, namely objects, encapsulation, and inheritance.
// Let’s look at what each of those characteristics means and whether Rust supports it.

// The book Design Patterns: Elements of Reusable Object-Oriented Software by Erich Gamma, Richard Helm,
// Ralph Johnson, and John Vlissides (Addison-Wesley Professional, 1994) colloquially referred to as The
// Gang of Four book, is a catalog of object-oriented design patterns. It defines OOP this way:

// Object-oriented programs are made up of objects. An object packages both data and the procedures that
// operate on that data. The procedures are typically called methods or operations.

// Using this definition, Rust is object oriented: structs and enums have data, and impl blocks provide
// methods on structs and enums. Even though structs and enums with methods aren’t called objects, they
// provide the same functionality, according to the Gang of Four’s definition of objects.

// Encapsulation that Hides Implementation Details
// Another aspect commonly associated with OOP is the idea of encapsulation, which means that the
// implementation details of an object aren’t accessible to code using that object. Therefore, the only
// way to interact with an object is through its public API; code using the object shouldn’t be able to
// reach into the object’s internals and change data or behavior directly. This enables the programmer
// to change and refactor an object’s internals without needing to change the code that uses the object.

// We discussed how to control encapsulation in Chapter 7: we can use the pub keyword to decide which
// modules, types, functions, and methods in our code should be public, and by default everything else
// is private. For example, we can define a struct AveragedCollection that has a field containing a
// vector of i32 values. The struct can also have a field that contains the average of the values in
// the vector, meaning the average doesn’t have to be computed on demand whenever anyone needs it. In
// other words, AveragedCollection will cache the calculated average for us. Listing 17-1 has the
// definition of the AveragedCollection struct:

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();

        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// The public methods add, remove, and average are the only ways to access or modify data in an instance of
// AveragedCollection. When an item is added to list using the add method or removed using the remove method,
// the implementations of each call the private update_average method that handles updating the average field
// as well.

// We leave the list and average fields private so there is no way for external code to add or remove items
// to the list field directly; otherwise, the average field might become out of sync when the list changes.
// The average method returns the value in the average field, allowing external code to read the average but
// not modify it.

// Because we’ve encapsulated the implementation details of the struct AveragedCollection, we can easily
// change aspects, such as the data structure, in the future. For instance, we could use a HashSet<i32>
// instead of a Vec<i32> for the list field. As long as the signatures of the add, remove, and average public
// methods stay the same, code using AveragedCollection wouldn’t need to change. If we made list public
// instead, this wouldn’t necessarily be the case: HashSet<i32> and Vec<i32> have different methods for
// adding and removing items, so the external code would likely have to change if it were modifying list
// directly.

// If encapsulation is a required aspect for a language to be considered object oriented, then Rust meets
// that requirement. The option to use pub or not for different parts of code enables encapsulation of
// implementation details.

// Inheritance as a Type System and as Code Sharing
// Inheritance is a mechanism whereby an object can inherit from another object’s definition, thus gaining
// the parent object’s data and behavior without you having to define them again.

// If a language must have inheritance to be an object-oriented language, then Rust is not one. There is no
// way to define a struct that inherits the parent struct’s fields and method implementations. However, if
// you’re used to having inheritance in your programming toolbox, you can use other solutions in Rust,
// depending on your reason for reaching for inheritance in the first place.

// You choose inheritance for two main reasons. One is for reuse of code: you can implement particular
// behavior for one type, and inheritance enables you to reuse that implementation for a different type.
// You can share Rust code using default trait method implementations instead, which you saw in Listing 10-14
// when we added a default implementation of the summarize method on the Summary trait. Any type implementing
// the Summary trait would have the summarize method available on it without any further code. This is
// similar to a parent class having an implementation of a method and an inheriting child class also having
// the implementation of the method. We can also override the default implementation of the summarize
// method when we implement the Summary trait, which is similar to a child class overriding the implementation
// of a method inherited from a parent class.

// The other reason to use inheritance relates to the type system: to enable a child type to be used in
// the same places as the parent type. This is also called polymorphism, which means that you can
// substitute multiple objects for each other at runtime if they share certain characteristics.

// Polymorphism
// To many people, polymorphism is synonymous with inheritance. But it’s actually a more general concept
// that refers to code that can work with data of multiple types. For inheritance, those types are
// generally subclasses.

// Rust instead uses generics to abstract over different possible types and trait bounds to impose
// constraints on what those types must provide. This is sometimes called bounded parametric polymorphism.

// Inheritance has recently fallen out of favor as a programming design solution in many programming
// languages because it’s often at risk of sharing more code than necessary. Subclasses shouldn’t
// always share all characteristics of their parent class but will do so with inheritance. This can
// make a program’s design less flexible. It also introduces the possibility of calling methods on
// subclasses that don’t make sense or that cause errors because the methods don’t apply to the
// subclass. In addition, some languages will only allow a subclass to inherit from one class,
// further restricting the flexibility of a program’s design.

// For these reasons, Rust takes a different approach, using trait objects instead of inheritance.
// Let’s look at how trait objects enable polymorphism in Rust.

// Using Trait Objects That Allow for Values of Different Types
// We are going to create a gui library

// Defining a Trait for Common Behavior
// To implement the behavior we want gui to have, we’ll define a trait named Draw that will have one
// method named draw. Then we can define a vector that takes a trait object. A trait object points to
// both an instance of a type implementing our specified trait as well as a table used to look up trait
// methods on that type at runtime. We create a trait object by specifying some sort of pointer, such
// as a & reference or a Box<T> smart pointer, then the dyn keyword, and then specifying the relevant
// trait. (We’ll talk about the reason trait objects must use a pointer in Chapter 19 in the section
// “Dynamically Sized Types and the Sized Trait.”) We can use trait objects in place of a generic or
// concrete type. Wherever we use a trait object, Rust’s type system will ensure at compile time that
// any value used in that context will implement the trait object’s trait. Consequently, we don’t need
// to know all the possible types at compile time.

// We’ve mentioned that in Rust, we refrain from calling structs and enums “objects” to distinguish them
// from other languages’ objects. In a struct or enum, the data in the struct fields and the behavior in
// impl blocks are separated, whereas in other languages, the data and behavior combined into one concept
// is often labeled an object. However, trait objects are more like objects in other languages in the
// sense that they combine data and behavior. But trait objects differ from traditional objects in that
// we can’t add data to a trait object. Trait objects aren’t as generally useful as objects in other
// languages: their specific purpose is to allow abstraction across common behavior.

// Below shows how to define a trait named Draw with one method named draw:
pub trait Draw {
    fn draw(&self);
}

// This syntax should look familiar from our discussions on how to define traits in Chapter 10. Next comes
// some new syntax: Below defines a struct named Screen that holds a vector named components. This
// vector is of type Box<dyn Draw>, which is a trait object; it’s a stand-in for any type inside a Box that
// implements the Draw trait.
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// On the Screen struct, we’ll define a method named run that will call the draw method on each of its
// components
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Now we’ll add some types that implement the Draw trait. We’ll provide the Button type. Again, actually
// implementing a GUI library is beyond the scope of this book, so the draw method won’t have any useful
// implementation in its body. To imagine what the implementation might look like, a Button struct might
// have fields for width, height, and label
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to draw a button
    }
}

// If someone using our library decides to implement a SelectBox struct that has width, height, and options
// fields, they implement the Draw trait on the SelectBox type as well:
use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // some code to draw a select box
    }
}

// Our library’s user can now write their main function to create a Screen instance. To the Screen instance,
// they can add a SelectBox and a Button by putting each in a Box<T> to become a trait object. They can then
// call the run method on the Screen instance, which will call draw on each of the components. Listing 17-9
// shows this implementation:
use gui::{Screen, Button};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

// When we wrote the library, we didn’t know that someone might add the SelectBox type, but our Screen implementation
// was able to operate on the new type and draw it because SelectBox implements the Draw trait, which means it
// implements the draw method.

// This concept—of being concerned only with the messages a value responds to rather than the value’s concrete
// type—is similar to the concept duck typing in dynamically typed languages: if it walks like a duck and quacks
// like a duck, then it must be a duck! In the implementation of run on Screen in Listing 17-5, run doesn’t need
// to know what the concrete type of each component is. It doesn’t check whether a component is an instance of a
// Button or a SelectBox, it just calls the draw method on the component. By specifying Box<dyn Draw> as the type
// of the values in the components vector, we’ve defined Screen to need values that we can call the draw method on.

// The advantage of using trait objects and Rust’s type system to write code similar to code using duck typing is that
// we never have to check whether a value implements a particular method at runtime or worry about getting errors if a
// value doesn’t implement a method but we call it anyway. Rust won’t compile our code if the values don’t implement
// the traits that the trait objects need.

// Trait Objects Perform Dynamic Dispatch
// Recall in the “Performance of Code Using Generics” section in Chapter 10 our discussion on the monomorphization
// process performed by the compiler when we use trait bounds on generics: the compiler generates nongeneric implementations
// of functions and methods for each concrete type that we use in place of a generic type parameter. The code that results
// from monomorphization is doing static dispatch, which is when the compiler knows what method you’re calling at compile
// time. This is opposed to dynamic dispatch, which is when the compiler can’t tell at compile time which method you’re
// calling. In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.

// When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t know all the types that might be
// used with the code that is using trait objects, so it doesn’t know which method implemented on which type to call.
// Instead, at runtime, Rust uses the pointers inside the trait object to know which method to call. There is a runtime
// cost when this lookup happens that doesn’t occur with static dispatch. Dynamic dispatch also prevents the compiler
// from choosing to inline a method’s code, which in turn prevents some optimizations. However, we did get extra flexibility
// in the code that we wrote in Listing 17-5 and were able to support in Listing 17-9, so it’s a trade-off to consider.

// Object Safety Is Required for Trait Objects
// You can only make object-safe traits into trait objects. Some complex rules govern all the properties that make a trait
// object safe, but in practice, only two rules are relevant. A trait is object safe if all the methods defined in the trait
// have the following properties:

// The return type isn’t Self.
// There are no generic type parameters.
// The Self keyword is an alias for the type we’re implementing the traits or methods on. Trait objects must be object
// safe because once you’ve used a trait object, Rust no longer knows the concrete type that’s implementing that trait.
// If a trait method returns the concrete Self type, but a trait object forgets the exact type that Self is, there is no
// way the method can use the original concrete type. The same is true of generic type parameters that are filled in with
// concrete type parameters when the trait is used: the concrete types become part of the type that implements the trait.
// When the type is forgotten through the use of a trait object, there is no way to know what types to fill in the generic
// type parameters with.

// An example of a trait whose methods are not object safe is the standard library’s Clone trait. The signature for the clone
// method in the Clone trait looks like this:
pub trait Clone {
    fn clone(&self) -> Self;
}

// The state pattern is an object-oriented design pattern. The crux of the pattern is that a value has some internal state,
// which is represented by a set of state objects, and the value’s behavior changes based on the internal state. The state
// objects share functionality: in Rust, of course, we use structs and traits rather than objects and inheritance. Each
// state object is responsible for its own behavior and for governing when it should change into another state. The value
// that holds a state object knows nothing about the different behavior of the states or when to transition between states.

// We’ll implement a blog post workflow in an incremental way
use blog::Post;

fn main() {
    let mut post = Post::new();
    
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

// Defining Post and Creating a New Instance in the Draft State
// Let’s get started on the implementation of the library! We know we need a public Post struct that holds some content,
// so we’ll start with the definition of the struct and an associated public new function to create an instance of Post,
// as shown in Listing 17-12. We’ll also make a private State trait. Then Post will hold a trait object of Box<dyn State>
// inside an Option<T> in a private field named state. You’ll see why the Option<T> is necessary in a bit.

// The State trait defines the behavior shared by different post states, and the Draft, PendingReview, and Published
// states will all implement the State trait. For now, the trait doesn’t have any methods, and we’ll start by defining
// just the Draft state because that is the state we want a post to start in.

// When we create a new Post, we set its state field to a Some value that holds a Box. This Box points to a new instance
// of the Draft struct. This ensures whenever we create a new instance of Post, it will start out as a draft. Because
// the state field of Post is private, there is no way to create a Post in any other state! In the Post::new function,
// we set the content field to a new, empty String.

// The add_text method takes a mutable reference to self, because we’re changing the Post instance that we’re calling
// add_text on. We then call push_str on the String in content and pass the text argument to add to the saved content.
// This behavior doesn’t depend on the state the post is in, so it’s not part of the state pattern. The add_text method
// doesn’t interact with the state field at all, but it is part of the behavior we want to support.

// Next, we need to add functionality to request a review of a post, which should change its state from Draft to PendingReview

// We give Post a public method named request_review that will take a mutable reference to self. Then we call an internal
// request_review method on the current state of Post, and this second request_review method consumes the current state
// and returns a new state.

// We’ve added the request_review method to the State trait; all types that implement the trait will now need to implement
// the request_review method. Note that rather than having self, &self, or &mut self as the first parameter of the method,
// we have self: Box<Self>. This syntax means the method is only valid when called on a Box holding the type. This syntax
// takes ownership of Box<Self>, invalidating the old state so the state value of the Post can transform into a new state.

// To consume the old state, the request_review method needs to take ownership of the state value. This is where the Option
// in the state field of Post comes in: we call the take method to take the Some value out of the state field and leave a
// None in its place, because Rust doesn’t let us have unpopulated fields in structs. This lets us move the state value out
// of Post rather than borrowing it. Then we’ll set the post’s state value to the result of this operation.

// We need to set state to None temporarily rather than setting it directly with code like self.state = self.state.request_review();
// to get ownership of the state value. This ensures Post can’t use the old state value after we’ve transformed it into a new state.

// The request_review method on Draft needs to return a new, boxed instance of a new PendingReview struct, which represents the
// state when a post is waiting for a review. The PendingReview struct also implements the request_review method but doesn’t do
// any transformations. Rather, it returns itself, because when we request a review on a post already in the PendingReview state,
// it should stay in the PendingReview state.

// Now we can start seeing the advantages of the state pattern: the request_review method on Post is the same no matter its state
// value. Each state is responsible for its own rules.

// We’ll leave the content method on Post as is, returning an empty string slice. We can now have a Post in the PendingReview
// state as well as in the Draft state, but we want the same behavior in the PendingReview state. Listing 17-11 now works up to
// line 10!

// Similar to request_review, if we call the approve method on a Draft, it will have no effect because it will return self. When
// we call approve on PendingReview, it returns a new, boxed instance of the Published struct. The Published struct implements the
// State trait, and for both the request_review method and the approve method, it returns itself, because the post should stay in
// the Published state in those cases.

// Now we need to update the content method on Post: if the state is Published, we want to return the value in the post’s conten
// field; otherwise, we want to return an empty string slice, as shown.

// We add a default implementation for the content method that returns an empty string slice. That means we don’t need to implement
// content on the Draft and PendingReview structs. The Published struct will override the content method and return the value in
// post.content

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft{})),
            content: String::new();
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

// Encoding States and Behavior as Types
// We’ll show you how to rethink the state pattern to get a different set of trade-offs. Rather than encapsulating the
// states and transitions completely so outside code has no knowledge of them, we’ll encode the states into different
// types. Consequently, Rust’s type checking system will prevent attempts to use draft posts where only published posts
// are allowed by issuing a compiler error.

// Let’s consider the first part of main in Listing 17-11:
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
}

// We still enable the creation of new posts in the draft state using Post::new and the ability to add text to the post’s
// content. But instead of having a content method on a draft post that returns an empty string, we’ll make it so draft
// posts don’t have the content method at all. That way, if we try to get a draft post’s content, we’ll get a compiler
// error telling us the method doesn’t exist. As a result, it will be impossible for us to accidentally display draft
// post content in production, because that code won’t even compile. Listing 17-19 shows the definition of a Post struct
// and a DraftPost struct, as well as methods on each:
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

// Example being used:
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content())
}