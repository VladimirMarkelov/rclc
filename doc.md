# RionaCalc

Mathematical expression calculator with transparent support of big integers, floats, common fractions, and complex numbers.

Please visit [homepage](https://github.com/VladimirMarkelov/rclc) to share your ideas, your suggestions or to open bugs.

## Table of Content

* [Features](#features)
* [Command Line Usage](#command-line-usage)
    * [Commands](#commands)
* [Type of Values](#type-of-values)
* [Operators](#operators)
    * [Operator priority](#operator-priority)
    * [Percentage operator](#percentage-operator)
* [Functions](#functions)
* [Special Functions](#special-functions)
* [Builtin Constants](#builtin-constants)
* [Special Variables](#special-variables)
* [Scripts](#scripts)

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

## Command Line Usage

```
$ rclc [expression] [options]
```

If the application is launched without arguments, it starts in interactive mode. When either expression or `--file` option is provided, the calculator evaluates it, prints the result or error, and exits. The calculator can read the expressions from pipe(e.g, `cat exprs.txt | rclc` or like `bc`: `echo "12/35" | rclc`.

* `-v`, `--version` - prints the application version and exits
* `--debug` - prints detailed information. E.g, if you provide a startup file to execute, without option the calculator prints only errors. While with the option enabled it prints all evaluated lines and their results
* `-i`, `--interactive` - forces interactive mode. By default, if either expression or file is provided, the calculator closes after calculating the final result. Option `-i` switches the calculator into interactive mode after loading a file or evaluating the provided expression. Note: if rclc detects that its output or input is piped (e.g, `echo "2*3" | rclc -i`), the option `-i` does nothing
* `-f`, `--file` - loads and evaluates the file with expression at startup. It may be useful to preload your own set of constants or variables to use in the further calculations. When `--file` is defined, the calculator evaluates the file line by line, skipping comments, and prints out only the result of the last evaluated line. The option supports multiple values: `rclc -f script1 -f script2`, in this case scripts are evaluated in the order of appearance in command line.

## Interactive Mode

After entering interactive mode, the calculator displays prompt `>` and waits for an expression or command. To calculate an arbitrary expression, type it and press key `Enter`. Pressing `Ctrl+C` exits the application.

### Commands

The list of supported commands:

* `quit` - exists the application (the same as pressing `Ctrl+C` but it exits with code `0`)
* `exit` - alias for `quit`
* `load <filename>` - loads and evaluates a script. It is useful to load script if you forgot to do it with option `--file`, or if you want to reset variables/constants by loading their original values from file. Another script use-case is execute a script that uses external variable, so by assigning different values to the variable you can recalculate the script with new initial values.

Please, see section "Scripts" for examples of how to effectively use scripts and commands together.

## Type of Values

All numbers supports using `_` as a group separator to improve readability: `1024.5678` is the same as `1_024.56_78`. The calculator does not have a specific type for boolean values, it treats a zero value for `false` and any other value as `true`.

* Integer numbers - big integers with arbitrary precision. They can be written as:
  - bare integer number: `1024`
  - integer number with exponent: `12e6` is the same as `12000000` but shorter. Note: do not use decimal point in this form, otherwise the number is treated as a float number even if its fractional part is zero. Example: `1.2e4`, in spite if being an integer number `12000`, it is treated as float number `12000.0` that main ruin the following calculations if you expected to get an exact result as a big integer value
  - Hexadecimals(starts with `0x` or `0X`): `0x1f`
  - Octals(starts with `0o` or `0O`): `0o65`
  - Binary numbers(starts with `0b` or `0B`): `0b101`
* Float numbers: 64-bit floating point numbers:
  - simple format with decimal point: `1234.98`
  - exponential format with decimal point: `12.4e7`
* Rational numbers: a common fractions that uses big integers to represent their numerators and denominators:
  - with integer part: `1\1\10` - one and one tenth
  - numerator and denominator only: `11\10` - the same number one and one tenth
* Complex numbers: a pair of 64-bit floating point numbers to represent imaginary and real part a complex number(Note: as a complex number marker you can use capital or lowercase `i` or `j`):
  - trailing marker: `1-2i`
  - marker in the middle: `1-i2`
* Angles: internally they are 64-bit float numbers. Separators: `°` or `d` - for degrees, `'` or `m` - for minutes, `"` or `s` - for seconds:
  - Full format with degrees, minutes, and seconds: `30°45'24"` or `30d45m24s` - in this format degrees and minutes must be integer numbers. Note: separators can be mixed - for convenience: `°` can be har to type fast, so `30d45'24"` is the same as `30°45'24"` etc
  - Degrees only: `30.75°` or `30.75d` - in this format degrees can be either integer or float point number.
* Variable: a word starts with a Latin letter from `a` to `z` and contains only Latin letters, digits, and underscore. Variable names are case-insensitive.

## Expressions

For convenience there are a few shortcuts when typing an expression:

* sometimes multiplication sign can be omitted if it is clear that it must be at that place: between values and brackets(`2(3+4)` -> `2 * (3+4)`), between brackets(`(1+2)(3+4)` -> `(1+2) * (3+4)`), between value and function names(`20sin(2)` -> `20 * sin(2)`)
* if a function wants one argument and it is a single value, brackets can be omitted: `sin cos 2` -> `sin(cos(2))`
* trailing closing brackets can be omitted: `3 * (2 + 10` -> `3 * (2 + 10)`

### Operators

Supported mathematical operators:

* `+` - add
* `-` - subtract or change value's sign
* `*` - multiply
* `/` - divide
* `%` - division remainder and percentage
* `//` - integer division (drops fractional part of the result after division and converts the result into big integer)
* `!` - factorial if added after a value or after a closing bracket, logical negation in other cases
* `**` - raise to arbitrary  power
* `<<` and `>>` - left and right bitwise shift
* `~` - bitwise NOT
* `&`, `^` and `|` - bitwise AND, XOR, and OR
* `&&` and `||` - logical AND and NOT
* `==`, `!=`, `>`, `<`, `>=`, and `<=` - comparison operators. Note: while operators like greater or less do not make sense for complex numbers, the calculator implements them for consistency. For complex numbers all operators compare separately real and imaginary parts (so, one complex number is greater than the other one only if its both real and imaginary parts are greater than corresponding parts of the other number)

#### Operator priority

Starting from the highest priority:

1. `!`(factorial)
2. `!`(logical NOT), `~`, `-`(unary minus), `+`(unary plus)
3. `**`
4. `<<`, `>>`
5. `*`, `/`, `//`, `%`(modulo operator)
6. `+`, `-`, `%`(percentage operator)
7. `&`, `^`
8. `|`
9. `&&`
10. `||`
11. comparison operators (`==`, `>`, etc)

#### Percentage operator

The percentage operator `%` is a unique one: it never works alone and its result depends on the previous operator on the same nesting level.
The calculator treats `%` as percentage operator only if:

- a previous operator exists and it is one of `+`, `-`, `*`, and `/`
- character `%` is the final character or it is followed by either a closing bracket or another operator

In all other cases, `%` is treated as a modulo operator.

Combinations of `%` with other operators:

1. `a + b %` - increase the number `a` by `b` percents
2. `a - b %` - decrease the number `a` by `b` percents
3. `a * b %` - calculate `b` percent of `a` (short for `a * b / 100`)
4. `a / b %` - calculate how many percents is `a` of `b` (short for `a / b * 100`)

Note: in the first three cases `b` is always converted to a real number. In the last case, the result is always a real number.

Examples:

- `10 + 30 + 50 %` = `60`. Because percentage operator is the same priority as `+`, the expression is calculated as `(10 + 30) + 50 %`
- `sqrt(-16) / sqrt(-4) %` = `200.0`. It demonstrates that you even can calculate percentage of complex numbers. That is useless most of the time. In this case `sqrt(-16)` is twice bigger than `sqrt(-4)` and it results in `200 %` or `200.0`
- `10+3i / 3-2i %` = `184.6153846153846`. It is an example of useless result due to complex numbers are incomparable. First, two complex numbers are divided, and then the result is converted to a real number.
- `10 + 30 + 0+5i %` = `40`. Percentage is a complex number, so it is converted to a real number before use(the real part of the complex number is used). The expression turns into `10 + 30 + 0 %`
- `10 + 30 ** 50 %` = `ERROR: too many operators`. The previous operator is `**`, so `%` is treated as the modulo operator and it requires an extra argument.
- `(10 + 30) %` = `ERROR: too many operators`. The previous operator does not exist(nesting level of `+` is deeper. That is why it is skipped), so `%` is treated as the modulo operator.

### Functions

Function names are case insensitive.

* Trigonometric: `sin`, `cos`, `tan`
* Inverse trigonometric: `asin`, `acos`, `atan`
* Square: `sqr`
* Square and cubic root: `sqrt` and `cbrt`
* Exponent: `exp`
* Natural logarithm: `ln`
* Absolute value: `abs`
* Sign of a value: `signum`
* Rounding functions: `round`, `trunc`, `ceil`, `floor`
* Convert any value to common fraction: `ratio`
* Fractional part of a float point number: `fract`
* Hyperbolic: `sinh`, `cosh`, `tanh`
* Inverse hyperbolic: `asinh`, `acosh`, `atanh`
* Function for complex numbers: `norm`(modulus), `conj`(conjugate), `re`(real part of a complex), `im`(imaginary part of a complex)
* Immediate if: `iif(condition;true_value;false_value)` returns `true_value` if the `condition` is true, and `false_value` otherwise
* Least common multiple and greatest common divisor: `lcm` and `gcd`. Both function accept arbitrary number of arguments (starting from 2 arguments)
* Conversion between radians and degrees: `rad`(degrees to radians) and `deg`(degrees to radians)
* N-th number of Fibonacci: `fibo`. At this moment the argument must be an integer number between 0 and 100000
* Prime numbers: `is_prime` - returns `1` if a number is a prime one, `next_prime` returns the smallest prime number greater than the function argument. Note: the functions return error for non-integer numbers, and the algorithm is not very fast, so checking if a number is prime may take a lot of time if the number is rather big
* Formatting integers: `hex`, `oct`, and `bin` display integer numbers in hexadecimal, octal, or binary representation. Note: the functions affects output only if they are the last used functions, e.g. `1+hex(10)` displays `11`, but `hex(1+10)` displays `0xb`
* Equation solver: `solve`(aliases are `roots` and `zeroes`) - calculates roots of a linear or quadratic equation. The result of the function is the first calculated root. Other roots are just printed.

### Special functions

* Gamma function - extension of factorial function. The calculated value is not accurate, the difference between result and exact value is less than 0.000000001%

### Builtin Constants

Constant names are case insensitive. All constants are 64-bit precision float numbers.

1. `PI` - 3.141...
2. `E` - natural logarithm base(Euler's constant) - 2.71828...
3. `PHI` - golden section - 1.618...

### Special Variables

1. `ans` - result of the last successfully evaluated expression

### Scripts

Script is a plain text file in UTF-8 format. It can loaded with command line option `--file` or using command `load` from interactive mode. Scripting support is very basic: a script can contain only expressions, comments, and variable assignments:

* Expression is what you type in interactive mode
* Comment is any line that starts with `//` or `#`
* Variable assignment is the line that starts with a variable name and following assignment sign(`=`). Variable name must start with a Latin letter, and contain only Latin letters, digits, and underscore. Example of valid line: `sec_per_hour = 60 * 60`

Note: variable names are case-insensitive. Names of existing constants, internal variables, and functions cannot be used as variable names.

#### Example of script usage

It is very simplified example but you will get the idea. Let's suppose you often convert Fahrenheit to Celsius degrees. Instead of typing the formula every time, you can create a script with content `(f-32)*5\9` (alternative version of content: `(ans-32)*5\9`). Save the script in the file `f2d`. In interactive mode execute two commands:

```
> f=13  // set variable f to 13 Fahrenheit
= 13
> load f2d
= -10\5\9 // -10 degrees and 5\9 Celsius
```

For the alternative case:

```
> 13  // this assigns 13 to internal ans variable
= 13
> load f2d
= -10\5\9
```

If you want to convert without entering interactive mode:

```
$ rclc "f=13" --file=f2d
-10\5\9
$
```
