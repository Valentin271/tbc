# Tiny BASIC compiler

This is a compiler for a subset of the [Tiny BASIC](https://en.wikipedia.org/wiki/Tiny_BASIC)
language. It also dumps a number of other files related to the compilation toolchain.

It uses my library [tiny-elf](https://github.com/Valentin271/tiny-elf/) to manipulate assembly and generate ELF files.

## Parser generator

This project uses [pest](http://pest.rs/) to generate a parser from a grammar.
The grammar is defined in [`src/parser/tinybasic.pest`](src/parser/tinybasic.pest).

## Running

You can run the project by typing:

```sh
cargo run -- <tinybasic file>
```

For example:

```sh
cargo run -- data/opt.tb
```

You can add the `-r` flag to directly run it. This will also append the return code to the output.

# Features

## Statements

This is an overview of the language, including limitations related to each statement.

| Statement        | Action                                                                                               |
| ---------------- | ---------------------------------------------------------------------------------------------------- |
| `PRINT`          | Prints an expression                                                                                 |
| `IF`, `ELSE`     | Classical conditional statement. The condition is limited compared to C-style languages, see NOTE 2. |
| `GOTO`, `GOSUB`  | Go to the specified line, unlike TinyBASIC , this does not support `GOTO <expression>`               |
| `INPUT`          | Stores an input into a variable, currently only one digit number are supported                       |
| `LET`            | Declare a variable. Variables cannot contain strings                                                 |
| `END` , `RETURN` | Ends the program, this is normally not the semantic for `RETURN`                                     |

NOTE: An expression is a string or an arithmetic expression. Expressions can contain variables,
and as such, an expression can be a variable. Strings cannot appear in an arithmetic expression.

NOTE 2: A condition is of the form `<operand> <relop> <operand>` where `<relop>` is a relational
operator and `<operand>` can be a number or a variable, it **cannot** be an expression.

NOTE 3: `INPUT` must really receive one and only one character. This means inputting from the
command line will not work as it also records a `\n`. Instead you can use `echo -n "5" | dump.elf`

## Optimizations

Assembly optimizations can be seen in [`optimize.rs`](src/optimize.rs). It is mostly converting
an instruction or set of instruction to faster ones.

Structural optimizations, and perhaps the most interesting ones are scattered throughout the code
in `Optimize` implementations. The major ones are in [`expr.rs`](src/syntax_tree/expr.rs) and
[`stmt.rs`](src/syntax_tree/stmt.rs). Respectively, they can optimize away arithmetic expressions
and conditions at compile time.

# Fun

This compiler outputs multiple files in addition to the executable binary. It actually outputs two
binaries:

- `udump.elf`, `u` standing for unoptimized, this is the functioning program without any
  optimization
- `dump.elf` is the final file that a compiler would output, this is an executable ELF file with
  optimizations

## Assembly

In addition to executable files, this compiler outputs the corresponding assembly code. As for
executables there is `udump.asm` and `dump.asm`.
These are also fully functioning, in fact, you can compile them with `nasm`. There is a `make asm`
rule to compile and link `dump.asm` into `asm.out`.

Interestingly, when compiling with `nasm` you'll notice the file is way bigger than with this
compiler (for `opt.tb`, 8.3K after stripping vs 493 bytes). This is mainly due to `nasm` doing its
job correctly, using multiple sections and aligning them. `xxd asm.out` will reveal the file is mainly zeros.

## [Graphviz](https://graphviz.org/)

For educational purposes, the parse tree, AST and OST (optimized syntax tree) are dumped using
Graphviz. Those are very interesting to compare, especially on `opt.rs` which will be stripped of
unused calculations and statements.

# Project overview

The code is partially commented. Here is a list of the main modules and files in order of interest.

- [`parser`](src/parser.rs) is the first step of the compiler, it contains the pest generated
  parser and the grammar
- [`syntax_tree`](src/syntax_tree.rs) holds the AST and its nodes. It handles their conversion from
  the parse tree and generation as assembly or Graphviz
- [`symbol_table`](src/symbol_table.rs) contains the symbol table and its related
- [`graphviz`](src/graphviz.rs) contains everything related to [Graphviz](https://graphviz.org/)

# Possible improvements

In no particular order.

- Handle string for `INPUT`
- Handle numbers greater than 9 for `INPUT`, it is non-trivial to convert integers to string and
  even harder from string to integer, read syscall only returns what you'd consider a string
- Put strings in variables
- Implement `GOSUB` and `RETURN` properly
- Remove unused assembly functions (`print`, `printn`, `read` in [`syntax_tree.rs`](src/syntax_tree.rs))
