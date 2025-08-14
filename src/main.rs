fn main() {
   hello_world();
   muttable();
   string();
   shadowing();
}

fn hello_world(){
     println!("Hello, world!");

    let x: i32 = 32;
    let _y: i32;    // _y if not initialised

    _y = 32;

    assert_eq!(x,_y);
    println!("success!");
    println!("The value of x is {} and value of y is {} ",x,_y);
}

fn muttable(){
    let mut x = 1;
    x += 2;

    assert_eq!(x,3);
    println!("success!");
}

fn string(){
    let x:&str= "hello";
    println!("{} world!",x);
    println!("### shadowing ###");
}

fn shadowing(){
    let x: i32 = 12;
    {
        let x = 5;
        assert_eq!(x,5);
    }
    
    assert_eq!(x,12);

    let x = 42;
    assert_eq!(x,42);
    println!("shadowing success!");
}