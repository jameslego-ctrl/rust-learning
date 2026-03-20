#[allow(unused_variables)]
#[allow(non_upper_case_globals)]
#[allow(unused)]

fn main() {
    // // variables are immutable by default in rust so the keyword `mut` is used to make it mutable
    let mut x = 6;
    println!("the value of x is {x}");

    x = 7;
    println!("The value of x is {x}");

    // constants : Constants aren't just immutable by default--, they are always immutable. Declared using the `const` keyword instead of `let`
    //              The type of the value must be annotated.
    const Three_Hours_In_Seconds: u32 = 60 * 60 * 3;

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x is {}", x);
    }

    println!("The value of x is {}", x);
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces : {}", spaces);

    let x = "rust";
    let x = x.len();
    println!("The length of x is {x}");
}
