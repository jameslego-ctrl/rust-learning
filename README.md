## Introduction to Rust
---
## commands

**rustup** : rust is installed and managed by the `rustup`

`rustup update` : updates the version and all the packages

`rustup self uinstall` : uinstall all the packages and the rust

`cargo init / cargo new` : create a rust project

`cargo run`: compile and run

`cargo build`  : build and compile

`rustc` : The traditional compiler

### 1. fn hello_world() :"hello world syntax"

- **println!()** : prints and create a new line

- **print!()** : print but doesn't create a new line
- {} this are the placeholder of a variable inside the println! macros
- A variable can be used only if have been initailised
- A fn should always have a snake_name naming convention
- If unitialised variable is used : Error
- if uintialised but unused : warning

---

### 2. fn muttable()
- By nature all rust variables are immutable in nature we need to declare it as `mut` to mark it as mutable


### 3.string()

### 4.shadowing ()
- you can declare a new variable with the same name as a previous variable
- shadowing allows a variable to declare to a diff datatype too eg. (i32 => string)
- it doesnt mean that they are typecasted , its just that they are both diff variables

### 5. [allow(unused_variables)] 
- this allows the compiler to use unused variables without throwing a warning
- this line should be top of the code generally above fn main()

### 6. destructuring()
- we can use a pattern with `let` to destruct a tuple to separate variables
- Tips: you can use shadowing or mutability

### 7. destructuring_assignments()
- Introduced in rust 1.59 version : we can use tuple , slice , and struct patterns as the left-hand side of an assignment


