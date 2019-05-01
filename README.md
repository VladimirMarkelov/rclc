## RionaCalc

Mathematical expression calculator with big integers, floats, common fractions, and complex numbers support. It can be launched in two modes: immediate, when rcalc gets an expression, calculates it, prints the result, and exits; and interactive, when rcalc displays a prompt and calculates every expression you type.

Suggestions, ideas, pull requests are very welcome. Thank you!

## Why yet another calculator

From time to time I need to evaluate an expression that contains values other than float point numbers: common fractions, complex numbers, arbitrary precision integer numbers. But majority of calculators works only with floating point numbers. Even if a calculator supports any of other types of numbers, it often requires switching to special modes. E.g, calculators that supported common fractions required to enter common-fraction mode to use them - in this mode other kinds of numbers are unavailable. Some calculators support complex numbers but do not do it transparently for a user. E.g, its documentation states that complex numbers are supported but a calculator fails on a evaluating square root of negative number with an error "Invalid argument". I wanted to have a calculator that allows me to mix any types of numbers in one expression and it is able to detect the correct type of the result and argument. 

Lesser requirements:

* evaluate trigonometric functions using radians and degrees without turning radians/degrees switch on and off
* a calculator that works in terminal either in interactive mode or is able to calculate an expression passed in command line and display the result. Yes, `bc` can do it. But its usage seems counterintuitive to me: instead of simple call `bc "expr"` it must be launched as `echo "expr" | bc`
* user-defined variables. Though, majority of calculators supports it out of the box

Very simple examples(`ans` - an special variable that holds the result of the last successful evaluation):

```
> sqrt(-2)  // square root of negative number
= 0.0+1.4142135623730952i
> sqr(ans) // square root of a complex number may produce real number
= -2.0000000000000006
> 345**12 // big integer in action
= 2843342266303054544082275390625
> 1\2 + 3\5  // one half and three fifth is one and one tenth
= 1\1\10
> sqr(3\5)  // square of a rational number is a rational number
= 9\25
> sin(90°) == sin(pi/2) // degrees and radians mixed in one expression, '°' can be replaced with 'd' for easier typing 
= 1
```

## Features

* No modes: all types of values can be transparently used in one expression. E.g, `(1\2 + 3\5) * 2-3i + sin(30d) + cos(0.25)` - multiply sum of two common fractions - one half and three fifth - by a complex number, add sine of 30 degrees and cosine of 0.25 radians. Spaces are added only for readability, they can be omitted
* Automatic selection of more appropriate argument type for a function: e.g, `sqrt(-4)` converts float number `-4` into complex one `-4+0i` and then calculates the result `0+2i`. The same is true for calculating logarithm for negative float numbers, and acos and asin for argument greater than `1.0`
* Automatic adding multiplication sign where it is omitted: e.g, `(1+2)(2+9)` is calculated as `(1+2)*(2+9)`
* Functions with a single-value argument do not require to enclose its argument into brackets: e.g, `sin cos 2` is calculated as `sin(cos(2))`
* The final closing brackets can be omitted: e.g, `(1+2)*(2+9` is the same as `(1+2)*(2+9)`
* Trigonometric functions work with radians and degrees. Bare numbers are treated as radians, degrees requires one or three suffixes. Two degrees formats: `20d30m50s` or `20°30'50"`. Minutes and seconds can be omitted, in this case degrees can be float number like `30.25d`. So, `sin(pi/2)` == `sin(90°)`
* Every number can include group separator `_` for readability - it is very useful when using big integers. `3_000.90_23` == `3000.9023`
* Both `.` and `,` are treated as decimal separators
* Function argument separator is `;`. If a function receives more arguments than it requires, the trailing arguments are dropped: e.g, `sqrt(11;12;13)` is the same as `sqrt(11)` 
* Regular fractions use `\` to separate its parts. They can be written with integer part or only with numerator and denominator, e.g `1\1\10` == `11\10`
* Two complex numbers formats: with marker at the end or in the middle. E.g, `1+2i` == `1+i2`. In addition, `j` can be used instead of `i` - but the calculator outputs always with `i`
* Hexadecimal(starts with `0x`), octal(starts with `0o`), and binary(starts with `0b`) numbers
* Basic variable and scripting support allows users to create their own constant libraries and preload them at calculator startup
* Commands in interactive mode(a very limited set at this moment): `quit` or `exit` close the calculator, and `load <filename>` - load the file and evaluate lines one by one, skipping comments, the last evaluated result is printed

Please, read the detailed [documentation here](docs.md).

## Installation

The application can be compiled from source, or installed using cargo:

```shell
$ cargo install rcalc
```

You need Rust compiler that supports Rust 2018 edition (Rust 1.31 or newer) to do it. If you want to upgrade existing rcalc, execute the following command:

```shell
$ cargo install rcalc --force
```

### Pre-compiled binaries

For Windows you can download pre-compiled binaries from [Release page](https://github.com/VladimirMarkelov/rcalc/releases).

* Windows binary works on Windows 7 or newer Windows.

### Known issues

- The calculator is not thoroughly tested, bugs may happen. Please, notify me about any issue
- While the calculator supports bitwise operations, it always displays a result as integer number. There is no way yet to display the result in hex or binary
- Float numbers with arbitrary precision are not supported yet - all floats are 64-bit float numbers. I am aware of rust-port of GNU GMP, but I do not want to use it at this moment - I remember having troubles trying to build the library on Windows
- The list of supported functions and constants is not long at this moment. I am going to add more later. If you need any function which rcalc does not have yet, please suggest it
