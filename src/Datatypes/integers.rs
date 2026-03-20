// Integer Overflow

// Each signed variant : can store numbers from −(2^(n − 1)) to 2^(n − 1) − 1 inclusive, where n is the number of bits that variant uses.
//                          So, an i8 can store numbers from −(2^7) to (2^7) − 1, which equals −128 to 127.

// Unsigned variants : can store numbers from 0 to (2^n) − 1, so a u8 can store numbers from 0 to (2^8) − 1, which equals 0 to 255.


fn main() {

// To explicitly handle the possibility of overflow, you can use these families of 
// methods provided by the standard library for primitive numeric types:

// checking_* method : 
// This method returns an Option<T>:
// 1.Some(val): If the addition is safe, it gives you the result.
// 2.None: If the addition would cause an overflow, it returns None to warn you.
    let x: u8 = 255;
    match x.checked_add(1) {                            // adds 1 to x and checks for oveflow, if overflown returns None so prints the error msgs
    Some(val) => println!("Result: {}", val),
    None => println!("Error: Overflow occurred!"),
    }

// wrapping_* method
    let y : u8 = 255;
    let result = y.wrapping_add(1);                     // adds 1 to y and checks for overflow , if overflown it wraps to 0,1,2,.... etc
    println!("The result : {result}");


// overflowing_* method
    let z : u8 = 255;
    let (result,has_overflown) = z.overflowing_add(2);  // adds 2 to z and checks for overflow, if overflown gives result and bool true else gives result and bool false
    println!("The result: {}, has overflown: {}",result, has_overflown);


// saturating_* method
    let x : u8 = 255;
    let result = x.saturating_add(1);               // adds 1 to x and checks for overflow , if overflown it gives the 
    println!("result: {}",result);                  // maximum possible value of the variant for eg. u8 will give 255
}                                                   // no matter how much we add to x
