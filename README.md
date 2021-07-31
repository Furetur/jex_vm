# jex_vm

Virtual machine that executes bytecode produced by [JexCompiler](https://github.com/Furetur/JexCompiler).

jex_vm is implemented on top of [extendable_vm](https://github.com/Furetur/extendable_vm) library.

## Human-readable Programming Language

jex_vm executes bytecode which is not human-readable.

Jex is a programming language that I am working on for the purpose of learning about compilers and virtual machines.
You can learn more about Jex [here](https://github.com/Furetur/JexCompiler) as well as compile it to jex_vm bytecode.


## About

This is a stack VM that supports

 - [x] Booleans, Strings, Ints and Functions
 - [x] Basic operations like addition, multiplication, concatenation, etc
 - [x] Conditional jumps, function calls and returns
 - [x] Exceptions that halt the machine and print the stack trace
 - [x] Heap allocated objects

## How to Run

You can download the latest version from the *Releases page*.
Or you can [build from source](#building-from-source).

### Linux

After you have the binary executable `jex_vm` you can run it:

```shell
./jex_vm path/to/bytecode
```

### Windows

After you have the binary executable `jex_vm.exe` you can run it:

```shell
./jex_vm.exe path/to/bytecode
```

### Run with logging

To run with logging you have to set the environment variable `RUST_LOG=jex_vm,extendable_vm`.
For example,

```shell
RUST_LOG=jex_vm,extendable_vm ./jex_vm path/to/bytecode
```

## Run Examples

You can run the [bytecode examples](examples) with jex_vm.

For instance, running [2_times_10.bytecode](examples/2_times_10.bytecode) should print 20.

## Instructions

`u8` represents an unsigned 8-bit integer

Name | Opcode (`u8`) | Arguments (name: type) | Stack (old → new) | Description
--- | --- | --- | --- | --- |
Constant | 0 | *i*: `u8` | [] → [value] | Loads onto stack the *i*-th constant from the constant pool
Null | 1 | | [] → [null] | Loads `null` onto stack
True | 2 | | [] -> [true] | Loads `true` onto stack
False | 3 | | [] -> [false] | Loads `false` onto stack
Pop | 4 | | [x, y] → [x] | Pops the last value from stack
Get local | 5 | *offset*: `u8` | [..., x] → [..., x, y] | Gets the `offset`-th operand in the current call frame and loads in onto stack
Set local | 6 | *offset*: `u8` | [..., x, ..., y] → [..., y, ...] | Pops the value and sets the `offset`-th operand in the current call frame
Get global | 7 | *identifier_i*: `u8` | [...] → [..., x] | Loads a global value onto stack by its identifier which it fetches from the constant pool by index = *identifier_i*
Define global | 8 | *identifier_i*: `u8` | [x] → [] | Sets a global value with the given identifier
Set global | 9 | *identifier_i*: `u8` | [x] → [] | Sets a global value with the given identifier
Print | 10 | | [x] → [] | Prints a value
Not | 11 | | [x] → [!x] | Logical NOT 
Equal | 12 | | [x, y] → [x == y] | Checks if 2 values are equal 
Greater | 13 | | [x, y] → [x > y] | Checks if first is greater than the second 
Less | 14 | |  [x, y] → [x < y] | Checks if first is less than the second 
Negate | 15 | | [x] → [-x] | Negates an integer 
Add | 16 | | [x, y] → [x + y] | Adds integers or concatenates strings 
Subtract | 17 | | [x, y] → [x - y] | Subtracts integers 
Multiply | 18 | | [x, y] → [x * y] |Multiplies integers 
Divide | 19 | | [x, y] → [x / y] | Divides integers 
Jump forward | 20 | *offset*: `u8` | | Jumps forward by `offset` bytes 
Jump forward if false | 21 | *offset*: `u8` | [x] → [] | Jumps forward by `offset` bytes if the value if `false`
Jump Backward | 22 | *offset*: `u8` | | Jumps backward by `offset` bytes 
Call | 23 | *arity*: `u8` | | Calls a function with `arity` arguments. For example, `CALL 3` will call `f(a, b, c)` when stack is `[f, a, b, c]`
Return | 24 | | Pops the last call frame and puts the returned value on top | Returns from the function ToString | 25 | | [x] → [string representation of x] | Converts a value to string
To string | 25 |  | [x] → [str(x)] | Converts value to string
Read line | 26 | | [] -> [x] | Suspends the VM and reads a like from STDIN
Parse int | 27 | | [x] -> [int(x)] | Parses string as int. If string cannot be parsed returns null
New instance | 28 | | [] → [x] | Creates an empty instance
Get field | 29 | *constant_id*: `u8` | [obj] -> [field_value] | Gets a field of obj: `obj[str_constant]`
Set field | 30 | *constant_id*: `u8` | [obj, value] -> [obj] | Sets a field of obj `obj[str_constant] = value`

## Bytecode format

This describes the format of the bytecode that the VM can read from the file.

### Notation

`struct`s are used as a way to demonstrate what each byte means. Each struct should be viewed as an array of bytes where
each value directly follow the previous (without padding and packing).

For example, struct `A` represents an array `[a1, a2, b]` where `a1` and `a2` correspond to `a: u16` and `b` to `b: u8`.

```rust
struct A {
    a: u16,
    b: u8
}
```

### Bytecode

Bytecode is an array of bytecode chunks. First chunks is a global script which will be run first, other chunks can be
called as function.

Each chunk has `n_constants` constants (constant pool) and `n_code_bytes`
executable bytes that contain instructions and their arguments.

```rust
struct Bytecode {
    chunks: [Chunk]
}

struct Chunk {
    n_constants: u8,
    constants: [Constant],
    // of `n_constants` size
    n_code_bytes: u16,
    code: [u8] // of `n_code_bytes` size
}
```

### Constants

Bytecode constants are literal values that are included in the code. There are 3 types of constants: ints, strings and
functions.

Each constant type has a unique `constant_type` which is used to distinguish it from the other types.

```rust
struct Constant {
    constant_type: u8,
    data: [u8]
}

// Constant := IntConstant | StringConstant | FunctionConstant

struct IntConstant {
    constant_type: u8,
    // always 0
    value: i32 // little endian
}

struct StringConstant {
    constant_type: u8,
    // always 1
    length: u16,
    utf8_data: [u8]
}

struct FunctionConstant {
    constant_type: u8,
    // always 2
    chunk_id: u8
}
```

### Calling functions

All chunks except for the first one can be called as a function with a `CALL` instruction.

Callable chunk must have these 2 constants:

* the first constant must be a function name (string)
* the second constant must be a function arity (int)

To call a chunk you need to load it onto stack with a `Constant` instruction, load some arguments onto stack and call it
with a `CALL arity` instruction.

## Building from source

### Build a development version

The executable will be located under `target/debug`.

```shell
cargo build
```

### Build a release version

The executable will be located under `target/release`.

```shell
cargo build --release
```

### Run tests

```shell
cargo test
```
