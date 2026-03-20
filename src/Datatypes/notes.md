# Data Types

**We’ll look at two data type subsets:** scalar and compound.

## scalar types
A scalar type represents a single value. Rust has four primary scalar types: 
- integers,
- floating-point numbers, 
- Booleans, and 
- characters.

## 1. **Integers** : 
*signed* (i8,i16,i32....) && *unsigned*(u8,u16,u32...): 

We can use any of these variants to declare the type of an integer value.

| Length | signed | unsigned|
| :--- | :--- | :--- |
|8 bit | i8 | u8 |
|16 bit | i16 | u16 |
|32 bit | i32 | u32 |
|64 bit | i64 | u64 |
|128bit | i128 | u128|
|Architecture Dependent | isize | usize |

### *signed* : 
- can be both positive and negative value e.g i8 : -128 to +127
- Each signed variant can store numbers from −(2n − 1) to 2n − 1 − 1 inclusive, where n is the number of bits that variant uses. 
- So, an i8 can store numbers from −(27) to 27 − 1, which equals −128 to 127.

### *unsigned* : 
- can only be positive value e.g u8: 0 to 255
- Unsigned variants can store numbers from 0 to 2n − 1, 
- so a u8 can store numbers from 0 to 28 − 1, which equals 0 to 255.

### Note: 
    ```bash
    Additionally, the isize and usize types depend on the architecture of the computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.


