# idk-man
## Stack based interpreted programing language in Rust

## Features
 - Pushing values (only integers for now)
 - Various arithmetic operations (+, -, /, %, *)
 - Stack manipulation operations (Dup)
 - Basic control flow (if, while)
 - Printing values to stdout (print)
 - Logical operations (<, >)

## Language usage
### Arithmetics
 - Add 34 and 35
```
34 35 +
```
### Stack manipulation
 - Duplicate 100 on the stack
```
100 dup
```
### Control flow
#### if
 - If 69 < 100
```
34 35 + 100 < if
303 print
end
```
#### while
 - A for loop
 ```
 69 while dup 0 > do
 dup print
 wend
 ```
