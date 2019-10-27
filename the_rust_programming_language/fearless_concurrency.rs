// Splitting the computation in your program into multiple threads can improve performance because
// the program does multiple tasks at the same time, but it also adds complexity. Because threads
// can run simultaneously, there’s no inherent guarantee about the order in which parts of your
// code on different threads will run. This can lead to problems, such as:

// - Race conditions, where threads are accessing data or resources in an inconsistent order

// - Deadlocks, where two threads are waiting for each other to finish using a resource the other
// thread has, preventing both threads from continuing

// - Bugs that happen only in certain situations and are hard to reproduce and fix reliably

// Programming languages implement threads in a few different ways. Many operating systems provide
// an API for creating new threads. This model where a language calls the operating system APIs to
// create threads is sometimes called 1:1, meaning one operating system thread per one language
// thread.

// Many programming languages provide their own special implementation of threads. Programming
// language-provided threads are known as green threads, and languages that use these green threads
// will execute them in the context of a different number of operating system threads. For this reason,
// the green-threaded model is called the M:N model: there are M green threads per N operating system
// threads, where M and N are not necessarily the same number.

// The green-threading M:N model requires a larger language runtime to manage threads. As such, the
// Rust standard library only provides an implementation of 1:1 threading. Because Rust is such a
// low-level language, there are crates that implement M:N threading if you would rather trade overhead
// for aspects such as more control over which threads run when and lower costs of context switching,
// for example.

// Now that we’ve defined threads in Rust, let’s explore how to use the thread-related API provided by
// the standard library.

// Creating a New Thread with spawn
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// Waiting for All Threads to Finish Using join Handles
// We can fix the problem of the spawned thread not getting to run, or not getting to run completely,
// by saving the return value of thread::spawn in a variable. The return type of thread::spawn is JoinHandle
// A JoinHandle is an owned value that, when we call the join method on it, will wait for its thread to
// finish. Listing 16-2 shows how to use the JoinHandle of the thread we created in Listing 16-1 and call
// join to make sure the spawned thread finishes before main exits:
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i)
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

// The two threads continue alternating, but the main thread waits because of the call to handle.join() and does
// not end until the spawned thread is finished. But let’s see what happens when we instead move handle.join()
// before the for loop in main, like this:
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// Since the join was called right after the first handle creation the output will no longer be alternating. It
// waits for the handle thread to finish:
// hi number 1 from the spawned thread!
// hi number 2 from the spawned thread!
// hi number 3 from the spawned thread!
// hi number 4 from the spawned thread!
// hi number 5 from the spawned thread!
// hi number 6 from the spawned thread!
// hi number 7 from the spawned thread!
// hi number 8 from the spawned thread!
// hi number 9 from the spawned thread!
// hi number 1 from the main thread!
// hi number 2 from the main thread!
// hi number 3 from the main thread!
// hi number 4 from the main thread!

// This code will not compile:
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

// Rust infers how to capture v, and because println! only needs a reference to v, the closure tries to borrow v.
// However, there’s a problem: Rust can’t tell how long the spawned thread will run, so it doesn’t know if the
// reference to v will always be valid.

// By adding the move keyword before the closure, we force the closure to take ownership of the values it’s using
// rather than allowing Rust to infer that it should borrow the values. The modification to Listing 16-3 shown
// in Listing 16-5 will compile and run as we intend:
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

// Using Message Passing to Transfer Data Between Threads
// One increasingly popular approach to ensuring safe concurrency is message passing, where threads or actors
// communicate by sending each other messages containing data. Here’s the idea in a slogan from the Go language
// documentation: “Do not communicate by sharing memory; instead, share memory by communicating.”

// One major tool Rust has for accomplishing message-sending concurrency is the channel, a programming concept that
// Rust’s standard library provides an implementation of. You can imagine a channel in programming as being like
// a channel of water, such as a stream or a river. If you put something like a rubber duck or boat into a stream,
// it will travel downstream to the end of the waterway.

// A channel in programming has two halves: a transmitter and a receiver. The transmitter half is the upstream
// location where you put rubber ducks into the river, and the receiver half is where the rubber duck ends up
// downstream. One part of your code calls methods on the transmitter with the data you want to send, and another
// part checks the receiving end for arriving messages. A channel is said to be closed if either the transmitter
// or receiver half is dropped.

// We create a new channel using the mpsc::channel function; mpsc stands for multiple producer, single consumer.
// In short, the way Rust’s standard library implements channels means a channel can have multiple sending ends
// that produce values but only one receiving end that consumes those values. Imagine multiple streams flowing
// together into one big river: everything sent down any of the streams will end up in one river at the end. We’ll
// start with a single producer for now, but we’ll add multiple producers when we get this example working.

// The mpsc::channel function returns a tuple, the first element of which is the sending end and the second element
// is the receiving end. The abbreviations tx and rx are traditionally used in many fields for transmitter and
// receiver respectively, so we name our variables as such to indicate each end. We’re using a let statement with
// a pattern that destructures the tuples; we’ll discuss the use of patterns in let statements and destructuring
// in Chapter 18. Using a let statement this way is a convenient approach to extract the pieces of the tuple returned
// by mpsc::channel.

use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hello there!");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
// Again, we’re using thread::spawn to create a new thread and then using move to move tx into the closure so the
// spawned thread owns tx. The spawned thread needs to own the transmitting end of the channel to be able to send
// messages through the channel.

// The transmitting end has a send method that takes the value we want to send. The send method returns a Result<T, E>
// type, so if the receiving end has already been dropped and there’s nowhere to send a value, the send operation will
// return an error. In this example, we’re calling unwrap to panic in case of an error. But in a real application, we
// would handle it properly: return to Chapter 9 to review strategies for proper error handling.

// Sending Multiple Values and Seeing the Receiver Waiting
// The code in Listing 16-8 compiled and ran, but it didn’t clearly show us that two separate threads were talking to
// each other over the channel. In Listing 16-10 we’ve made some modifications that will prove the code in Listing 16-8
// is running concurrently: the spawned thread will now send multiple messages and pause for a second between each message.

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });


    for received in rx {
        println!("Got: {}", received);
    }
}
// Creating Multiple Producers by Cloning the Transmitter (above)
// Earlier we mentioned that mpsc was an acronym for multiple producer, single consumer. Let’s put mpsc to use and expand
// the code in Listing 16-10 to create multiple threads that all send values to the same receiver. We can do so by cloning
// the transmitting half of the channel, as shown in Listing 16-11:

// Shared-State Concurrency
// Message passing is a fine way of handling concurrency, but it’s not the only one. Consider this part of the slogan from
// the Go language documentation again: “communicate by sharing memory.”

// What would communicating by sharing memory look like? In addition, why would message-passing enthusiasts not use it and
// do the opposite instead?

// In a way, channels in any programming language are similar to single ownership, because once you transfer a value down a
// channel, you should no longer use that value. Shared memory concurrency is like multiple ownership: multiple threads can
// access the same memory location at the same time. As you saw in Chapter 15, where smart pointers made multiple ownership
// possible, multiple ownership can add complexity because these different owners need managing. Rust’s type system and
// ownership rules greatly assist in getting this management correct. For an example, let’s look at mutexes, one of the
// more common concurrency primitives for shared memory.

// Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one thread to access some data at any given time.
// To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex’s lock. The
// lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data.
// Therefore, the mutex is described as guarding the data it holds via the locking system.

// Mutexes have a reputation for being difficult to use because you have to remember two rules:

// You must attempt to acquire the lock before using the data.
// When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.

// As an example of how to use a mutex, let’s start by using a mutex in a single-threaded context:
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

// As with many types, we create a Mutex<T> using the associated function new. To access the data inside the mutex, we use
// the lock method to acquire the lock. This call will block the current thread so it can’t do any work until it’s our turn
// to have the lock.

// The call to lock would fail if another thread holding the lock panicked. In that case, no one would ever be able to get
// the lock, so we’ve chosen to unwrap and have this thread panic if we’re in that situation.


// After we’ve acquired the lock, we can treat the return value, named num in this case, as a mutable reference to the data
// inside. The type system ensures that we acquire a lock before using the value in m: Mutex<i32> is not an i32, so we must
// acquire the lock to be able to use the i32 value. We can’t forget; the type system won’t let us access the inner i32
// otherwise.

// As you might suspect, Mutex<T> is a smart pointer. More accurately, the call to lock returns a smart pointer called MutexGuard,
// wrapped in a LockResult that we handled with the call to unwrap. The MutexGuard smart pointer implements Deref to point at
// our inner data; the smart pointer also has a Drop implementation that releases the lock automatically when a MutexGuard goes
// out of scope, which happens at the end of the inner scope in Listing 16-12. As a result, we don’t risk forgetting to release
// the lock and blocking the mutex from being used by other threads because the lock release happens automatically.

// Sharing a Mutex<T> Between Multiple Threads
// This code will not compile:
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// Maybe writing it like this will help see the error state:
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();

        *num += 1;
    });
    handles.push(handle);

    let handle2 = thread::spawn(move || {
        let mut num2 = counter.lock().unwrap();

        *num2 += 1;
    });
    handles.push(handle2);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// Aha! The first error message indicates that counter is moved into the closure for the thread associated with handle.
// That move is preventing us from capturing counter when we try to call lock on it and store the result in num2 in the
// second thread! So Rust is telling us that we can’t move ownership of counter into multiple threads. This was hard to
// see earlier because our threads were in a loop, and Rust can’t point to different threads in different iterations of
// the loop. Let’s fix the compiler error with a multiple-ownership method we discussed in Chapter 15.

// Multiple Ownership with Multiple Threads
// In Chapter 15, we gave a value multiple owners by using the smart pointer Rc<T> to create a reference counted value.
// Let’s do the same here and see what happens. We’ll wrap the Mutex<T> in Rc<T> in Listing 16-14 and clone the Rc<T>
// before moving ownership to the thread. Now that we’ve seen the errors, we’ll also switch back to using the for loop,
// and we’ll keep the move keyword with the closure.
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
// Wow, the error message from the above code is very wordy! Here are some important parts to focus on: the first
// // inline error says `std::rc::Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely.

// Unfortunately, Rc<T> is not safe to share across threads. When Rc<T> manages the reference count, it adds to the
// count for each call to clone and subtracts from the count when each clone is dropped. But it doesn’t use any
// concurrency primitives to make sure that changes to the count can’t be interrupted by another thread. This could
// lead to wrong counts—subtle bugs that could in turn lead to memory leaks or a value being dropped before we’re
// done with it. What we need is a type exactly like Rc<T> but one that makes changes to the reference count in a
// thread-safe way.

// Atomic Reference Counting with Arc<T>
// Fortunately, Arc<T> is a type like Rc<T> that is safe to use in concurrent situations. The a stands for atomic,
// meaning it’s an atomically reference counted type. Atomics are an additional kind of concurrency primitive that
// we won’t cover in detail here: see the standard library documentation for std::sync::atomic for more details. At
// this point, you just need to know that atomics work like primitive types but are safe to share across threads.

// You might then wonder why all primitive types aren’t atomic and why standard library types aren’t implemented
// to use Arc<T> by default. The reason is that thread safety comes with a performance penalty that you only want
// to pay when you really need to. If you’re just performing operations on values within a single thread, your code
// can run faster if it doesn’t have to enforce the guarantees atomics provide.
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
// This will print "Result: 10"

// Similarities Between RefCell<T>/Rc<T> and Mutex<T>/Arc<T>

// You might have noticed that counter is immutable but we could get a mutable reference to the value inside it; this means
// Mutex<T> provides interior mutability, as the Cell family does. In the same way we used RefCell<T> in Chapter 15 to allow
// us to mutate contents inside an Rc<T>, we use Mutex<T> to mutate contents inside an Arc<T>.

// Another detail to note is that Rust can’t protect you from all kinds of logic errors when you use Mutex<T>. Recall in
// Chapter 15 that using Rc<T> came with the risk of creating reference cycles, where two Rc<T> values refer to each other,
// causing memory leaks. Similarly, Mutex<T> comes with the risk of creating deadlocks. These occur when an operation needs
// to lock two resources and two threads have each acquired one of the locks, causing them to wait for each other forever.
// If you’re interested in deadlocks, try creating a Rust program that has a deadlock; then research deadlock mitigation
// strategies for mutexes in any language and have a go at implementing them in Rust. The standard library API documentation
// for Mutex<T> and MutexGuard offers useful information.

// Extensible Concurrency with the Sync and Send Traits
// Interestingly, the Rust language has very few concurrency features. Almost every concurrency feature we’ve talked about so
// far in this chapter has been part of the standard library, not the language. Your options for handling concurrency are not
// limited to the language or the standard library; you can write your own concurrency features or use those written by others.

// However, two concurrency concepts are embedded in the language: the std::marker traits Sync and Send.
// Allowing Transference of Ownership Between Threads with Send

// The Send marker trait indicates that ownership of the type implementing Send can be transferred between threads. Almost every
// Rust type is Send, but there are some exceptions, including Rc<T>: this cannot be Send because if you cloned an Rc<T> value
// and tried to transfer ownership of the clone to another thread, both threads might update the reference count at the same
// time. For this reason, Rc<T> is implemented for use in single-threaded situations where you don’t want to pay the thread-safe
// performance penalty.

// Therefore, Rust’s type system and trait bounds ensure that you can never accidentally send an Rc<T> value across threads
// unsafely. When we tried to do this in Listing 16-14, we got the error the trait Send is not implemented for Rc<Mutex<i32>>.
// When we switched to Arc<T>, which is Send, the code compiled.

// Any type composed entirely of Send types is automatically marked as Send as well. Almost all primitive types are Send, aside
// from raw pointers, which we’ll discuss in Chapter 19.

// The Sync marker trait indicates that it is safe for the type implementing Sync to be referenced from multiple threads. In other
// words, any type T is Sync if &T (a reference to T) is Send, meaning the reference can be sent safely to another thread. Similar
// to Send, primitive types are Sync, and types composed entirely of types that are Sync are also Sync.

// The smart pointer Rc<T> is also not Sync for the same reasons that it’s not Send. The RefCell<T> type (which we talked about
// in Chapter 15) and the family of related Cell<T> types are not Sync. The implementation of borrow checking that RefCell<T>
// does at runtime is not thread-safe. The smart pointer Mutex<T> is Sync and can be used to share access with multiple threads
// as you saw in the “Sharing a Mutex<T> Between Multiple Threads” section.

// Implementing Send and Sync Manually Is Unsafe
// Because types that are made up of Send and Sync traits are automatically also Send and Sync, we don’t have to implement those
// traits manually. As marker traits, they don’t even have any methods to implement. They’re just useful for enforcing invariants
// related to concurrency.

// Manually implementing these traits involves implementing unsafe Rust code. We’ll talk about using unsafe Rust code in Chapter
// 19; for now, the important information is that building new concurrent types not made up of Send and Sync parts requires careful
// thought to uphold the safety guarantees. The Rustonomicon has more information about these guarantees and how to uphold them.
