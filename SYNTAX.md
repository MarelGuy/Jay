# Language syntax ideas and rules

## BASIC RULES

 - The main file should have a main function (see "FUNCTIONS")
 - Every line should end with a `;`

## FUNCTIONS

The main function can be of two type.
Either:
```rust
fn main(): void {}
```
Or
```rust
let main = (): void {}
```

One can also use anonymous functions:
```ts
(): type {}
```

One can return a value in three ways:

```rust
x // no semicolon at the end of the function
return x;
return_if y < x {x} else {y};
```

## VARIABLES

Variables can be declared in three ways:
```ts
// Value needs to be known at compile time, cannot be changed after declaration
const x: int = 0;
// Value doesn't need to be known at compile time, cannot be changed after declaration
let x: int = 0;
// Value doesn't need to be known at compile time, can be changed after declaration
var x: int = 0;
```

### Types

Variables have 5 primitive types:
```
int
float
char
str
string
bool
```
To create a new type see "CUSTOM TYPES"

### Values

A proper int will be automatiaclly assigned by the compiler ranging from i8 to i64

`int: -2^63-1 to 2^63-1`

A proper unsigned int will be automatiaclly assigned by the compiler ranging from u8 to u64

`uint: 0 to 2^64-1`

`float: f64`

A str is a type of string that is not known at compile time and will be automaticaclly allocated on the heap

`str: "Hello, world!"`

A string is a type of string that is known at compile time and will be automaticaclly allocated on the stack

`string: "Hello, world!"`

A char is a single byte type that contains a single letter

`char: 'a'`

A bool is a single byte type that contains either 11111111 (true) or 00000000 (false)

`bool: true or false`

## CUSTOM TYPES

One can create a new type by typing:
```ts
type {
    name: string;
    x: int;
    y: uint;
    new: (name: string, x: int, y: unit) {
        return type {name, x, y};
    };
};
```

## LOOPS

There are three types of loops 
```rust
for i in 0...10 {};

// see IFELSE-Conditions to know more about conditions
while condition {};
```

every loop can be stop with the keyword `break`
a loop cycle an also be skipped with the keyword `skip`

## IFELSE

One can create a ifelse statement in two ways:
```rust
if condition {
} else if condition {
} else {};
```
```ts
condition ? "" : "";
```
one can't do an elseif with ternary operators

### Conditions

conditions have special properties:
```rust
// normal condition
a == b
// condition AND condition
a == b && c == d
// condition OR condition
a == b || c == d
// condition XOR condition
a == b ^^ c == d
```
```rust
a == (b || c)
a == (b && c)
a == (b ^^ c)
```

## PATTERN MATCHING

```rust
match {
    1 {},
    2 {},
    3 {},
    _ {},
}
```

## IMPORT AND EXPORT

```ts
export {a, b, c}
export {x as a, y as b, z as c}
```

```ts
import lib from lib
import {a, b, c} from lib
```

## COMMENTS

```rust
// Comments

/*
Block Comments
*/
```
