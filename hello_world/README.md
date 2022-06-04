# Hello, World!
My first app in `rust` by rust book.

The [link](https://doc.rust-lang.org/book/ch01-02-hello-world.html) that was followed. 

## Something new!

### Compile rust program
- Before running a Rust program, you must compile it using the Rust compiler by entering the rustc command and passing it the name of your **source file**, like this:
```bash
$ rustc main.rs
```

- Run program compiled program.
```bash
$ ./main
```

### Rust Code
- functions in `rust` writes with `fn` `func_name`, brackets for args `()` and brackets for body `{}`
- here is exemple of main func which returns nothing
```rust
fn main() {

}
```
- also every rust program starts from `main` function

- the main function body has this code
```rust
    println!("Hello, world!");
```

In this function have four thing
1. In rust syntax uses four spaces ' ', not a tab '\t'.
1. `println!` calls a Rust macro. If it called a function instead, it would be entered as println (without the `!`). For now, you just need to know that using a `!` means that you’re calling a macro instead of a normal function, and that macros don’t always follow the same rules as functions.
1. you see the "Hello, world!" string. We pass this string as an argument to println!, and the string is printed to the screen.
1. we end the line with a semicolon (;), which indicates that this expression is over and the next one is ready to begin. Most lines of Rust code end with a semicolon.

### Rust code formatter
- Rust have formatter (`rustfmt`) which format you code. Example:
```bash
$ rustfmt main.rs
```
- Also `rustc` can format to.

> Also many thinks writed in book.