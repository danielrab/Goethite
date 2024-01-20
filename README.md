# Goethite
Goethite is a general purpose programming language, the main initial inspiration for which was Rust (even if it deviated a lot as different design ideas were explored), as such, this overview explains some things by analogy or contrast with Rust.
## Goals
- Be ease to learn: Goethite aims be easy to learn even for someone who never programmed before. This means
    - Few keywords and operators to learn.
    - A compiler that doesn't get in your way.
    - Syntax that you can guess the meaning of in a lot of cases.
    - Things generally have the same meaning in any context.
- Be flexibility and powerful: Goethite aims to be useful for a variety of application with minimal boilerplate and powerful abstractions. This is achieved through
    - Compile time computations.
    - Treating types as values.
    - Macro system that allows you to deconstruct an expression into parts and construct a new one.
    - The ability to create custom syntax or override existing syntax for part of the file if you need it.
- Be predictability and safe: Goethite aims to make you sure that your code won't suddenly break. This means
    - Your code doesn't break from adding a dependency, ever.
    - If two items conflict, it will be known at definition, not at usage, always.
    - If a function isn't marked as producing an error or an effect, it won't, unless you specifically opt out of marking that in a given module.
    - Things that happen in some scope stay in that scope unless explicitly brought out.
## Non-goals
- Be as fast as possible: Goethite generally tries to allow you to write fast code, but is ok with taking small hits to performance it order to further the goals stated above.
- Be similar to existing languages: While for completely arbitrary decisions, Goethite will generally choose the way of doing things that resembles what the established languages are doing, the merit of each feature is evaluated independently of how familiar it will be to someone used to existing languages.

# Core concepts
## Variables
There are two keywords for declaring variables in Goethite: `const` and `var`.

`var` allows the value to be changed with the `set` keyword, but only allows usage after declaration, for example:
```
var a = 1
print a
set a = 2
print a
```
will print 1 then print 2, while
```
print(a)
var a = 1
```
will produce a compilation error

`const` allows the value to be used in the entire block, including before declaration, as long as it doesn't create dependency cycles, but doesn't allow for it to be changed, for example:
```
print a
const a = 1
```
will print 1, while
```
const a = 1
set a = 2
```
will produce a compilation error

## Functions
Anonymous functions, also known as closures (or in some languages lambdas) have the form `argument => result`, where `argument` can be an identifier or a pattern (technically an identifier is the simplest pattern), and `result` is an expression.
They can be passed to another function or assigned to a variable (usually `const`), which allows to use them as a normal function, and for recursion.
If a pattern doesn't match the value provided, the call fails. The compiler must ensure that this doesn't happen. You can merge functions by providing callback with `|`
```
const fib = x => x:match(
    | 0 => 1
    | 1 => 1
    | x => fib(x-1) + fib(x-2)
)
```

## Types
Types of expressions can be annotated with the `of` operator, for example `String of foo()` will not compile if `foo()` can return something other than `String`