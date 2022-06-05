# Programming a Guessing Game
> The [link](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) that was followed.

Let’s jump into Rust by working through a hands-on project together! This chapter introduces you to a few common Rust concepts by showing you how to use them in a real program. You’ll learn about `let`, `match`, `methods`, `associated functions`, using `external crates`, and more! In the following chapters, we’ll explore these ideas in more detail. In this chapter, you’ll practice the fundamentals.

We’ll implement a classic beginner programming problem: a guessing game. Here’s how it works: the program will generate a random integer between 1 and 100. It will then prompt the player to enter a guess. After a guess is entered, the program will indicate whether the guess is too low or too high. If the guess is correct, the game will print a congratulatory message and exit.