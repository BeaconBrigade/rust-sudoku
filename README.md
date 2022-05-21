# rust_sudoku
The sudoku solver I made in C, but remade in rust to get some practice using the language.

I'm trying to implement the same backtracking algorithm that I used before to solve the puzzle.
I'll only be using the standard rust library.

**Using**

The program can be invoked from the command line:
`cargo run -- <FILEPATH/TO/SUDOKUPUZZLE.txt> [-p<delay]`

The aforementioned .txt file will should use the following guidelines:

1. Numbers that are known will just be numbers (1-9)
2. Letters and the number 0 will be interpretted as blank squares
3. Whitespace will be ignored
4. The program will only read the first 81 non-whitespace characters

The `-p<delay` option will tell the program to print all partial solutions. The `<delay>` is how many milliseconds are
delayed between each print.

Example file:

1  6 _  _  _  _  _  _  _

2  _  _  0  _  _  _  _  _

3  _  _  _  _  _  _  8  _

4  _  _  _  _  _  _  _  _

5  _  _  0  _  _  _  _  _

6  _  _  _  _  _  _  _  _

7  _  2  _  _  b  _  _  _

8  _  _  _  _  _  _  _  _

9  _  _  _  q  _  8  _  _

Converted into (as image)

![sudoku_example_cropped](https://user-images.githubusercontent.com/100320298/169367764-f1fe46eb-6326-4101-97dc-d9b1d3471e8b.png)

**Changes**

Besides this project being rewritten in a new language, the biggest difference is how I implemented the tree. Because Rust 
requires extra safety, and doesn't really approve of the self-referential structures I employed in C, I had to make some
changes. Instead of storing Nodes freely on the heap, they are stored in a Vec. Every new Node is pushed onto the Vec. 
So, within each Node structure instead of holding pointers to its children, it stores indices to its children in the Vec.
Many other things were kept to work in a similar, if not the same, way as my C implementation.

**Rust**

Working with Rust for a first project was actually a LOT of fun. Coming from building a few projects in C/C++ it was pretty
shocking how nice the compiler was. It gives you advice on EVERYTHING. I also had fun with how easy cargo was to use 
compared to creating your own Makefiles by hand. I also got to play with rustfmt as well. All of these tools made it lots
of fun starting using Rust. The most important part too, I didn't have to worry about any memory leaks, dangling or wild pointers.
All of the heap allocations were taken care of. I have to say, again, that it was a LOT of fun using Rust.

![Ferris the Crab](https://rustacean.net/assets/rustacean-orig-noshadow.png)

