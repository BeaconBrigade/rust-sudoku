# Rust Sudoku Solver
The sudoku solver I made in C, but remade in rust to get some practice using the language.

I'm trying to implement the same backtracking algorithm that I used before to solve the puzzle.
I'll only be using the standard rust library.

## Usage

The program can be invoked from the command line:
`cargo run -- <FILEPATH/TO/SUDOKUPUZZLE.txt> [-p<delay]`

The aforementioned .txt file will be read as follows:

1. The numbers 1-9 will be interpreted as the hints.
2. Letters and the number 0 will be interpreted as blank squares
3. Whitespace will be ignored
4. The program will only read the first 81 non-whitespace characters

The `-p<delay` option will tell the program to print all partial solutions. The `<delay>` is how many milliseconds are
delayed between each print.

Example file:

![sudoku puzzle in txt format](https://user-images.githubusercontent.com/100320298/169670900-7a61b8ca-f7b6-4db2-bb67-336299de8ecb.png)

Converted into (as image)

![sudoku puzzle converted](https://user-images.githubusercontent.com/100320298/169367764-f1fe46eb-6326-4101-97dc-d9b1d3471e8b.png)

## Demo

[Full video on Youtube](https://youtu.be/miwEKd8_TLc)

![Rust_Sudoku_Solver](https://user-images.githubusercontent.com/100320298/171497375-5417d01c-4cb7-4a7e-a344-17aa9aaccf21.gif)


## Changes

Besides this project being rewritten in a new language, the biggest difference is how I implemented the tree. Because Rust 
requires extra safety, and doesn't really approve of the self-referential structures I employed in C, I had to make some
changes. Instead of storing Nodes freely on the heap, they are stored in a Vec. Every new Node is pushed onto the Vec. 
So, within each Node structure instead of holding pointers to its children, it stores indices to its children in the Vec.
Many other things were kept to work in a similar, if not the same, way as my C implementation.

## Rust

Working with Rust for a first project was actually a LOT of fun. Coming from building a few projects in C/C++ it was pretty
shocking how nice the compiler was. It gives you advice on EVERYTHING. I also had fun with how easy cargo was to use 
compared to creating your own Makefiles by hand. I also got to play with rustfmt as well. All of these tools made it lots
of fun starting using Rust. The most important part too, I didn't have to worry about any memory leaks, dangling or wild pointers.
All of the heap allocations were taken care of. I have to say, again, that it was a LOT of fun using Rust.

![Ferris the Crab](https://rustacean.net/assets/rustacean-orig-noshadow.png)

