# Full-Directory-Regex-using-Grub-in-Rust

This is lab exercise 1 in CS 155: Compiler Construction. I used the crates `Glob` and `Regex` to search through a specified directory and output all the files where their contents match the regex string. We were tasked to create two ways of implementation:

Imperative Way:
- Usage of loops and stack pushes

Functional Way:
- Usage of built-in Rust functions

To run, go to repository directory and execute the command in terminal: `cargo run --bin <imp/fp> -- "<directory>" "<regex>"`

This will output the paths of all the files with matching regex.

Sample Demo: This directory contains the source code for a reduced version of linux. I want to print all the `.c` files containing the word `fork()`
![image](https://user-images.githubusercontent.com/23215457/224646430-c3f46c36-f7f1-4680-bd5f-8a913b21ba79.png)

The first command is the `imperative` way and the second command is the `functional` way.
