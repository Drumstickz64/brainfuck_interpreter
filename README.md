# Brainfuck interpreter

A brainfuck interpreter written in Rust.

## What is Brainfuck

According to Wikipedia

> Brainfuck is an esoteric programming language created in 1993 by Urban Müller.
> Notable for its extreme minimalism, the language consists of only eight simple commands, a data pointer and an instruction pointer. While it is fully Turing complete, it is not intended for practical use, but to challenge and amuse programmers. Brainfuck requires one to break commands into microscopic steps.

## Example

```rust
use brainfuck_interpreter::BrainfuckInterpreter;

let program = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
let cell_array_size = 30_000;
let bfi = BrainfuckInterpreter::new(program, cell_array_size);
bfi.run();
```
