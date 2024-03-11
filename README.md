    ⠀⠀⣴⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣦⡀⠀  
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
    This is a simple example of a calculator written to demonstrate the syntax and abilities of Matador.
    Written by Eric (Skepz) <skepz.dev@gmail.com>
*/

// Introduction message
println "Welcome to the Matador Calculator!"

// Get the first number
print "Enter the first number: "
let first = readint

// Get the operator
print "Enter the operator (+, -, *, /): "
let operator = readln

// Get the second number
print "Enter the second number: "
let second = readint

// Perform the calculation
if operator == "+" {
    println "The result is: " + (first + second)
} else if operator == "-" {
    println "The result is: " + (first - second)
} else if operator == "*" {
    println "The result is: " + (first * second)
} else if operator == "/" {
    println "The result is: " + (first / second)
} else {
    println "Invalid operator!"
}
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
- [x] variable type casting
- [ ] literal variable typecasting (i.e. `5 as float`)
- [x] type checking
- [ ] loops
  - [x] while
  - [x] loop
  - [ ] for
  - [ ] break
  - [ ] continue
- [x] boolean logic
  - [x] and
  - [x] or
  - [x] leading not
- [x] scopes
  - [x] global (lives for lifetime of program unless dropped)
  - [x] local / block (lives for lifetime of block unless dropped)
- [x] arrays
- [ ] maps
- [ ] ranges
- [ ] in keyword
- [ ] inc / dec operators (++ / --)
- [ ] Assign and operate operators (+=, -=, *=, /=, %=)
- [ ] functions
  - [ ] replace built in keywords like readln and print with functions
- [ ] structs
- [ ] multi-file support
- [ ] loading and running other matador programs (maybe done through a function call)
- [ ] expose API for creating libraries
- [ ] expose lexer for linting and syntax highlighting
- [ ] better error messages
- [ ] optimize and refactor
