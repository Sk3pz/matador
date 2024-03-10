    ⠀⠀⣴⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀  ⢸⣦⡀⠀  
    ⠀⢸⣿⣧⣀⣀⠀⠀⠀⢀⣀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡀⠀⠀⠀⢀⣀⣼⣿⡧⠀  
    ⠀⠈⠻⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⠿⠁⠀  
    ⠀⠀⠀⠀⠙⠛⠿⠿⠿⠿⣿⠀⠀⠀⠀⠀⠀⠀⠀⣿⡿⠿⠿⠿⠛⠋⠁⠀⠀⠀  
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀  
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣤⡄⠀⠀⠀⠀⠀⠀⢀⣤⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀  
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈X⠿⠀⠀⠀⠀⠀⠀⠿X⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀  
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀  
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀  
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀  
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⡆⠀⠀⢠⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀  
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀⠀⠀⠀⣿⠆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀  
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠷⣦⣤⡾⠋⠀ 
# Matador
Matador is a general purpose interpreted programming language for embedding in games and other applications.
### Code Example (Currently runs):
```
/*
Matador Programming Language Test
Written by Eric (Skepz) <skepz.dev@gmail.com>
*/

// variable declarations
let x = 5
let y = 10

// type tests
let neg = -1
let flt = 1.0
let str = "Hello, World!"
let b = true

let result = x + y // addition

// printlning
println result

// reassignment
x = 2
y = x * y + result

println y
// free y from memory as it is not used again
drop y
println str

// conditionals
if true {
    println "This is true"
} else {
    println "This is false"
}

if x == 2 {
   println "x is 2"
} else if x == 3 {
   println "x is 3"
} else {
   println "x is not 2 or 3"
}

println "This is Matador (" + b + ")! 🐂"
```
### Output:
```
15
35
Hello, World!
This is true
x is 2
This is Matador (true)! 🐂
```
See [tests](./matador_tests) for more examples.  

Todo:
- [x] basic arithmetic
  - [x] addition / subtraction
  - [x] multiplication / division
  - [x] modulo
  - [x] unary negation
  - [x] negation
  - [x] power
  - [x] comparison
  - [x] bitwise / logical
  - [x] parentheses / order of operations
- [x] comments
- [x] types
- [x] variables
- [x] printing
- [x] conditionals (if/else)
- [x] input
- [x] type casting
- [x] type checking
- [ ] loops
  - [x] while
  - [ ] loop
  - [ ] for
  - [ ] break
  - [ ] continue
- [ ] boolean logic
  - [x] and
  - [x] or
  - [ ] leading not
- [x] scopes
  - [x] global (lives for lifetime of program unless dropped)
  - [x] local / block (lives for lifetime of block unless dropped)
- [ ] functions
  - [ ] replace built in keywords like readln and print with functions
- [ ] arrays
- [ ] maps
- [ ] ranges
- [ ] structs
- [ ] multi-file support
- [ ] loading and running other matador programs
- [ ] expose API for creating libraries
- [ ] expose lexer for linting and syntax highlighting
- [ ] optimize and refactor
