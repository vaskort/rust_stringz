# rust_stringz

Hello, fellow Rustaceans! Welcome to `rust_stringz`! 😃

This is a simple string manipulation library for Rust that's about as useful as a chocolate teapot. 🍫☕

## Description

`rust_stringz` is the brainchild of a JavaScript developer who had one too many cups of coffee and thought, "Hey, why not learn Rust?" So, here we are. This library is basically the equivalent of a 'Hello, World!' program, but for someone who's already well-acquainted with JavaScript and wanted to delve into the wonderful (and slightly intimidating) world of Rust.

This library is like attempting to organize a philosophical debate about Socrates in the middle of a rock concert in Camden Town — it's not the usual fare, could be drowned out by the louder aspects, but if you really need it, it's there.

With version 0.2.0, this library now does three things: converts a string to lowercase, converts a string to uppercase, and counts occurrences of a substring within a string. You might be thinking, "Well, I can do that with the Rust Standard Library!" And you'd be 100% correct. This library is akin to setting up a new tearoom at Trafalgar Square — the Brits have already got it covered, thanks! But remember, it's all in the spirit of learning! 🎓

## Usage

First, add `rust_stringz` to your Cargo.toml:

```toml
[dependencies]
rust_stringz = "0.2.0"
```

Then import the library and start using it:

```rust
extern crate rust_stringz;

let my_string = "Hello, Rust! Rust is fantastic!";
println!("{}", rust_stringz::to_lowercase(my_string)); // prints "hello, rust! rust is fantastic!"
println!("{}", rust_stringz::to_uppercase(my_string)); // prints "HELLO, RUST! RUST IS FANTASTIC!"
let count = rust_stringz::count_occurences(my_string, "Rust"); // counts the occurrences of "Rust"
println!("{}", count); // prints "2"
```

## License

This project is licensed under the MIT License. So you're free to use this library in any way you want, although I'm not sure why you'd want to...
