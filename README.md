# rust_stringz

Hello, fellow Rustaceans! Welcome to `rust_stringz`! 😃

This is a simple string manipulation library for Rust that's about as useful as a chocolate teapot. 🍫☕

## Description

`rust_stringz` is the brainchild of a JavaScript developer who had one too many cups of coffee and thought, "Hey, why not learn Rust?" So, here we are. This library is basically the equivalent of a 'Hello, World!' program, but for someone who's already well-acquainted with JavaScript and wanted to delve into the wonderful (and slightly intimidating) world of Rust.

This library is like attempting to perform Aeschylus' tragedies in the London Underground during rush hour: unusual, probably unneeded, but a spectacle nonetheless!

Currently, this library only does two simple things: converts a string to lowercase, and converts a string to uppercase. You might be thinking, "Well, I can do that with the Rust Standard Library!" And you'd be 100% correct. This library is akin to setting up a new tearoom at Trafalgar Square — the Brits have already got it covered, thanks! But remember, it's all in the spirit of learning! 🎓

## Usage

First, add `rust_stringz` to your Cargo.toml:

```toml
[dependencies]
rust_stringz = "0.1.0"
```

Then import the library and start using it:

```rust
extern crate rust_stringz;

let my_string = "Hello, Rust!";
println!("{}", rust_stringz::to_lowercase(my_string)); // prints "hello, rust!"
println!("{}", rust_stringz::to_uppercase(my_string)); // prints "HELLO, RUST!"
```

## License

This project is licensed under the MIT License. So you're free to use this library in any way you want, although we're not sure why you'd want to...

