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


