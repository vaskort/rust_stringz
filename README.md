# rust_stringz

Hello, fellow Rustaceans! Welcome to `rust_stringz`! 😃

This is a simple string manipulation library for Rust that's about as useful as a chocolate teapot. 🍫☕

## Description

`rust_stringz` is the brainchild of a JavaScript developer who had one too many cups of coffee and thought, "Hey, why not learn Rust?" So, here we are. This library is basically the equivalent of a 'Hello, World!' program, but for someone who's already well-acquainted with JavaScript and wanted to delve into the wonderful (and slightly intimidating) world of Rust.

This library is like attempting to organize a philosophical debate about Socrates in the middle of a rock concert in Camden Town — it's not the usual fare, could be drowned out by the louder aspects, but if you really need it, it's there.

"With version 1.0.0, this library now does four things. You might be thinking, "Well, I can do some of that with the Rust Standard Library!" And you'd be 100% correct. This library is akin to forming a punk rock band in the midst of a classical music festival. It's unexpected, it's loud, but hey, it sure makes things interesting! Remember, it's all in the spirit of learning and making some noise! 🎓"

| Function Name     | Description                                           |
|-------------------|-------------------------------------------------------|
| `to_lowercase`    | Converts a string to lowercase                        |
| `to_uppercase`    | Converts a string to uppercase                        |
| `count_occurrences` | Counts occurrences of a substring within a string     |
| `is_palindrome`   | Checks if a string (ignoring spaces, punctuation, and case) is a palindrome |

## Usage

First, add `rust_stringz` to your Cargo.toml:

```toml
[dependencies]
rust_stringz = "1.0.0"
```

Then import the library and start using it:

```rust
extern crate rust_stringz;

let my_string = "Hello, Rust! Rust is fantastic!";
println!("{}", rust_stringz::to_lowercase(my_string)); // prints "hello, rust! rust is fantastic!"
println!("{}", rust_stringz::to_uppercase(my_string)); // prints "HELLO, RUST! RUST IS FANTASTIC!"
let count = rust_stringz::count_occurrences(my_string, "Rust"); // counts the occurrences of "Rust"
println!("{}", count); // prints "2"
let palindrome = rust_stringz::is_palindrome("A man, a plan, a canal, Panama"); // checks if the string is a palindrome
println!("{}", palindrome); // prints "true"
```

## License

This project is licensed under the MIT License. So you're free to use this library in any way you want, although I'm not sure why you'd want to...
