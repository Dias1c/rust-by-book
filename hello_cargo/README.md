## Hello, Cargo!
> The [link](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) that was followed.

I've learned so far about Cargo:
- We can create a project using `cargo new`.
- We can build a project using `cargo build`.
- We can build and run a project in one step using `cargo run`.
- We can build a project without producing a binary to check for errors using `cargo check`.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the **target/debug** directory.

When your project is finally ready for release, you can use cargo `build --release` to compile it with optimizations. This command will create an executable in **target/release** instead of target/debug. 

## Cargo as Convention
With simple projects, Cargo doesn’t provide a lot of value over just using rustc, but it will prove its worth as your programs become more intricate. With complex projects composed of multiple crates, it’s much easier to let Cargo coordinate the build.

Even though the hello_cargo project is simple, it now uses much of the real tooling you’ll use in the rest of your Rust career. In fact, to work on any existing projects, you can use the following commands to check out the code using Git, change to that project’s directory, and build:
```bash
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```
