# Rust Sudoku Solver
The sudoku solver I made in C, but remade in rust to get some practice using the language.

I'm trying to implement the same backtracking algorithm that I used before to solve the puzzle.
I'll only be using the standard rust library.

## v0.2 Update:

After a few months learning more about rust, I've updated the program to be a bit more idiomatic and 
robust. I've added `bpaf` and `anyhow` to make a better cli. I've also added features to add an input
and output file. They fallback to stdin/stdout respectively (allowing data to be piped in or redirected
into the program. I also added multiple printing types including `simple`, `multiline` and `bordered` 
(previous method). `simple` is everything printed onto the same line, `multiline` is everything still
printed in a grid but without the borders around each square and `bordered` which prints borders around
the puzzle and every 3 x 3 area. Anyways, back to the original post which seems more like a blog now.

## Installation

- cargo
```shell
$ cargo install rust-sudoku
```
- Prebuilt binaries

Get a prebuilt binary from the releases.

## Usage

```shell
$ rust-sudoku --help
```

```text
Solve Sudoku problems *blazingly fast*.
Accepts input from a text file to fill in the puzzle. Each number (1-9) is interpreted
as that number in the puzzle. A zero or any other letter is considered a blank space.
Any whitespace is ignored.

Usage: [-i FILE] [-o FILE] [--style STYLE] [[-p]] [-d DELAY]

Available options:
    -i, --input <FILE>    Location of puzzle to read.
    -o, --output <FILE>   Output file to write solution to. Leave blank to write to stdout.
        --style <STYLE>   Print puzzle with nice borders, options include `simple`, `multiline` and `bordered`
    -p, --print-partials  Print each partial solution to the console as the program runs.
    -d, --delay <DELAY>   Add delay between each iteration in ms (useful when using `--print-partials`).
    -h, --help            Prints help information
    -V, --version         Prints version information
```

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

