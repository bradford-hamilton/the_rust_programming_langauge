fn main() {
    // IpAddrKind is now a custom data type that we can use elsewhere in our code
    enum IpAddrKind {
        V4,
        V6,
    }

    // We can create instances of each of the two variants of IpAddrKind like this:
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Note that the variants of the enum are namespaced under its identifier, and we use a double colon to
    // separate the two. The reason this is useful is that now both values IpAddrKind::V4 and IpAddrKind::V6
    // are of the same type: IpAddrKind. We can then, for instance, define a function that takes any
    // IpAddrKind and we can call this function with either variant:
    fn route(ip_kind: IpAddrKind) {};

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // You can also put data directly inside each enum variant
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // Even more complex ones:
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // Here are those data structures:
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct

    // You can also implement methods on an enum just like structs
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // Rust has an extremely powerful control flow operator called match that allows you to compare
    // a value against a series of patterns and then execute code based on which pattern matches.
    // Patterns can be made up of literal values, variable names, wildcards, and many other things.
    // The power of match comes from the expressiveness of the patterns and the fact that the
    // compiler confirms that all possible cases are handled.
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    // Another useful feature of match arms is that they can bind to the parts of the values that match
    // the pattern. This is how we can extract values out of enum variants.
    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }

    // In the previous section, we wanted to get the inner T value out of the Some case when using Option<T>;
    // we can also handle Option<T> using match as we did with the Coin enum! Instead of comparing coins,
    // we’ll compare the variants of Option<T>, but the way that the match expression works remains the same.
    // Let’s say we want to write a function that takes an Option<i32> and, if there’s a value inside, adds
    // 1 to that value. If there isn’t a value inside, the function should return the None value and not
    // attempt to perform any operations.

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // There’s one other aspect of match we need to discuss. Consider this version of our plus_one function
    // that has a bug and won’t compile:
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }
    // We didn’t handle the None case, so this code will cause a bug. Luckily, it’s a bug Rust knows how to catch.

    // The _ placeholder
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    // The _ pattern will match any value. By putting it after our other arms, the _ will match all the possible
    // cases that aren’t specified before it. The () is just the unit value, so nothing will happen in the _
    // case. As a result, we can say that we want to do nothing for all the possible values that we don’t list
    // before the _ placeholder.

    // Using match:
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // Using if let:
    if let Some(3) = some_u8_value {
        println!("three");
    }

    // Using match:
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // Using if let and else:
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

}