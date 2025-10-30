fn main (){
    let logical:bool = true;
    
    let a_float:f64 = 1.0; // Regular annotation
    let an_integer = 5i32; // Suffix annotation

    let default_float = 3.0; // f64
    let default_integer = 7; // i32


    // A type can also be inferred from context
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed
    let mut mutable = 12;
    mutable = 21;

    // Error! The type of a variable can't be changed
    // mutable = true;

    // Variables can be overwritten with shadowing
    let mutable = true;

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
   
    // Compound types -- tuples and arrays

      // Array signature consists of Type T and length as [T; length].
    let my_array:[i32;5] = [1,2,3,4,5];

    // Tuple is a collection of values of different types 
    // and is constructed using parentheses ().
    let my_tuple =(5u32, 1u8, true, -5.04f32);


    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    

 

}