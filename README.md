# rust_sudoku
The sudoku solver I made in C, but remade in rust to get some practice using the language.

I'm trying to implement the same backtracking algorithm that I used before to solve the puzzle.
I'll only be using the standard rust library.

**Using**

The program can be invoked from the command line:
`cargo run -- <FILEPATH/TO/SUDOKUPUZZLE.txt>`

The aforementioned .txt file will should use the following guidelines:

1. Numbers that are known will just be numbers (1-9)
2. Letters and the number 0 will be interpretted as blank squares
3. Whitespace will be ignored
4. The program will only read the first 81 non-whitespace characters

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
