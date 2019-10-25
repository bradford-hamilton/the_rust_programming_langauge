// Rust’s standard library includes a number of very useful data structures called collections. Most other
// data types represent one specific value, but collections can contain multiple values. Unlike the built-in
// array and tuple types, the data these collections point to is stored on the heap, which means the amount
// of data does not need to be known at compile time and can grow or shrink as the program runs. Each kind
// of collection has different capabilities and costs, and choosing an appropriate one for your current
// situation is a skill you’ll develop over time. In this chapter, we’ll discuss three collections that are
// used very often in Rust programs:

// - A vector allows you to store a variable number of values next to each other.

// - A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter
//   we’ll talk about it in depth.

// - A hash map allows you to associate a value with a particular key. It’s a particular implementation of
//   the more general data structure called a map.

// The first collection type we’ll look at is Vec<T>, also known as a vector. Vectors allow you to store
// more than one value in a single data structure that puts all the values next to each other in memory.
// Vectors can only store values of the same type. They are useful when you have a list of items, such
// as the lines of text in a file or the prices of items in a shopping cart.

// Creation of a new empty vector:
let v: Vec<i32> = Vec::new();

// In more realistic code, Rust can often infer the type of value you want to store once you insert values,
// so you rarely need to do this type annotation. It’s more common to create a Vec<T> that has initial values,
// and Rust provides the vec! macro for convenience. The macro will create a new vector that holds the values
// you give it. Because we’ve given initial i32 values, Rust can infer that the type of v is Vec<i32>, and
// the type annotation isn’t necessary.
let v = vec![1, 2, 3];

// Push some values onto a vector:
let mut v = Vec::new();
v.push(5);
v.push(6);

// Like any other struct, a vector is freed when it goes out of scope
{
    let v = vec![1, 2, 3, 4];
    // do stuff with v
} // <- v goes out of scope and is freed here

// You can access elements in a vector with either bracket notation or using "get". Bracket notation will
// panic if it attempts to reference a non-existent element. Get will return None without panicing
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}

// Iterate over a vector:
let v = vec![100, 32, 57];

for i in &v {
    println!("{}", i);
}

// Iterate over mutable references
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}

// At the beginning of this chapter, we said that vectors can only store values that are the same type. This
// can be inconvenient; there are definitely use cases for needing to store a list of items of different
// types. Fortunately, the variants of an enum are defined under the same enum type, so when we need to
// store elements of a different type in a vector, we can define and use an enum!

// For example, say we want to get values from a row in a spreadsheet in which some of the columns in the row
// contain integers, some floating-point numbers, and some strings. We can define an enum whose variants will
// hold the different value types, and then all the enum variants will be considered the same type: that of
// the enum. Then we can create a vector that holds that enum and so, ultimately, holds different types.

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];

// What Is a String?
// We’ll first define what we mean by the term string. Rust has only one string type in the core language, which
// is the string slice str that is usually seen in its borrowed form &str. In Chapter 4, we talked about string
// slices, which are references to some UTF-8 encoded string data stored elsewhere. String literals, for example,
// are stored in the program’s binary and are therefore string slices.

// The String type, which is provided by Rust’s standard library rather than coded into the core language, is a
// growable, mutable, owned, UTF-8 encoded string type. When Rustaceans refer to “strings” in Rust, they usually
// mean the String and the string slice &str types, not just one of those types. Although this section is largely
// about String, both types are used heavily in Rust’s standard library, and both String and string slices are
// UTF-8 encoded.

// Rust’s standard library also includes a number of other string types, such as OsString, OsStr, CString, and
// CStr. Library crates can provide even more options for storing string data. See how those names all end in
// String or Str? They refer to owned and borrowed variants, just like the String and str types you’ve seen
// previously. These string types can store text in different encodings or be represented in memory in a different
// way, for example.

// Creating a new string
let mut s = String::new();

// Start with some data in a string
let data = "initial contents";
let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();

// Or
let s = String::from("initial contents");

// Remember that strings are UTF-8 encoded, so we can include any properly encoded data in them
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שָׁלוֹם");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");

// Modifying a string
let mut s = String::from("foo");
s.push_str("bar"); // push a string slice

let mut s = String::from("lo");
s.push('l'); // push a single character

// String concatenation
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

// First, s2 has an &, meaning that we’re adding a reference of the second string to the first string because
// of the s parameter in the add function: we can only add a &str to a String; we can’t add two String values
// together. But wait—the type of &s2 is &String, not &str, as specified in the second parameter to add.

// The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String argument into
// a &str. When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..]. We’ll
// discuss deref coercion in more depth in Chapter 15. Because add does not take ownership of the s parameter,
// s2 will still be a valid String after this operation.

// Second, we can see in the signature that add takes ownership of self, because self does not have an &. This
// means s1 in Listing 8-18 will be moved into the add call and no longer be valid after that. So although
// let s3 = s1 + &s2; looks like it will copy both strings and create a new one, this statement actually takes
// ownership of s1, appends a copy of the contents of s2, and then returns ownership of the result. In other
// words, it looks like it’s making a lot of copies but isn’t; the implementation is more efficient than copying.

// If we need to concatenate multiple strings, the behavior of the + operator gets unwieldy:
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;

// So we can use the format! macro:
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);

// A String is a wrapper over a Vec<u8>. Let’s look at some an example string:
let len = String::from("Hola").len();

// In this case, len will be 4, which means the vector storing the string “Hola” is 4 bytes long. Each of these
// letters takes 1 byte when encoded in UTF-8. But what about the following line? (Note that this string begins
// with the capital Cyrillic letter Ze, not the Arabic number 3.)

let len = String::from("Здравствуйте").len();
// Asked how long the string is, you might say 12. However, Rust’s answer is 24: that’s the number of bytes it
// takes to encode “Здравствуйте” in UTF-8, because each Unicode scalar value in that string takes 2 bytes of
// storage. Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar
// value. To demonstrate, consider this invalid Rust code:

let hello = "Здравствуйте";
let answer = &hello[0];
// What should the value of answer be? Should it be З, the first letter? When encoded in UTF-8, the first byte
// of З is 208 and the second is 151, so answer should in fact be 208, but 208 is not a valid character on its own.
// Returning 208 is likely not what a user would want if they asked for the first letter of this string; however,
// that’s the only data that Rust has at byte index 0. Users generally don’t want the byte value returned, even
// if the string contains only Latin letters: if &"hello"[0] were valid code that returned the byte value, it would
// return 104, not h. To avoid returning an unexpected value and causing bugs that might not be discovered immediately,
// Rust doesn’t compile this code at all and prevents misunderstandings early in the development process.

// Slicing Strings
// Indexing into a string is oftfen a bad idea because it’s not clear what the return type of the string-indexing
// operation should be: a byte value, a character, a grapheme cluster, or a string slice. Therefore, Rust asks you to
// be more specific if you really need to use indices to create string slices. To be more specific in your indexing
// and indicate that you want a string slice, rather than indexing using [] with a single number, you can use []
// with a range to create a string slice containing particular bytes:
let hello = "Здравствуйте";
let s = &hello[0..4];

// Here, s will be a &str that contains the first 4 bytes of the string. Earlier, we mentioned that each of these
// characters was 2 bytes, which means s will be Зд.

// What would happen if we used &hello[0..1]? The answer: Rust would panic at runtime in the same way as if an
// invalid index were accessed in a vector

// You can also iterate over a string with different methods:
for c in "नमस्ते".chars() {
    println!("{}", c);
}

// And
for b in "नमस्ते".bytes() {
    println!("{}", b);
}

// Hash Maps
// Just like vectors, hash maps store their data on the heap.
// You can create an empty hash map with new and add elements with insert.
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// Another way of constructing a hash map is by using the collect method on a vector of tuples, where each tuple
// consists of a key and its value. The collect method gathers data into a number of collection types, including
// HashMap. For example, if we had the team names and initial scores in two separate vectors, we could use the
// zip method to create a vector of tuples where “Blue” is paired with 10, and so forth. Then we could use the collect
// method to turn that vector of tuples into a hash map
use std::collections::HashMap;

let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
// The type annotation HashMap<_, _> is needed here because it’s possible to collect into many different data structures
// and Rust doesn’t know which you want unless you specify. For the parameters for the key and value types, however, we
// use underscores, and Rust can infer the types that the hash map contains based on the types of the data in the vectors

// For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String,
// the values will be moved and the hash map will be the owner of those values
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point, try using them and compiler will be angry!

// We can get a value out of the hash map by providing its key to the get method
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name)

// We can iterate over each key/value pair in a hash map in a similar manner as we do with vectors, using a for loop:
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
  println!("{}: {}", key, value)
}

// If we insert a key and a value into a hash map and then insert that same key with a different value, the value associated with
// that key will be replaced. Even though the code in Listing 8-24 calls insert twice, the hash map will only contain one
// key/value pair because we’re inserting the value for the Blue team’s key both times.
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores); // {"Blue": 25} -> second insert overwrote the first

// Only Inserting a Value If the Key Has No Value
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores); // {"Yellow": 50, "Blue": 10}

// Updating a value based on the old value
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
// This code will print {"world": 2, "hello": 1, "wonderful": 1}. The or_insert method actually returns a mutable
// reference (&mut V) to the value for this key. Here we store that mutable reference in the count variable, so in
// order to assign to that value, we must first dereference count using the asterisk (*). The mutable reference goes
// out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.

// By default, HashMap uses a “cryptographically strong”1 hashing function that can provide resistance to Denial of Service
// (DoS) attacks. This is not the fastest hashing algorithm available, but the trade-off for better security that comes with
// the drop in performance is worth it. If you profile your code and find that the default hash function is too slow for your
// purposes, you can switch to another function by specifying a different hasher. A hasher is a type that implements the
// BuildHasher trait. We’ll talk about traits and how to implement them in Chapter 10. You don’t necessarily have to
// implement your own hasher from scratch; crates.io has libraries shared by other Rust users that provide hashers implementing
// many common hashing algorithms.