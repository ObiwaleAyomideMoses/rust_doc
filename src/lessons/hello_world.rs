use std::fmt;

struct Structure(i32);
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}



fn main(){


    let x = 5+ /*90 *5+*/ 6;
    println!("Hello, world! The value of x is: {}", x);

    println!("{} days in some months.", 31);
    println!("{0}, this is {1}, {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over}");

    println!("Base 10:               {}", 69420);
    println!("Base 2 (binary):               {:b}", 69420);
    println!("Base 8 (octal):                {:o}", 69420);
    println!("Base 16 (hexadecimal):         {:x}", 69420);
    println!("Base 16 (hexadecimal, upper):  {:X}", 69420);
    println!("{number:0>5}", number=1);
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");


    // For unprintable types, we can use the {:?} marker to print the debug representation
    #[derive(Debug)]
    struct DebugPrintable(i32);
    println!("{:?}", DebugPrintable(3));

    // Another example

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    };
    

    let peter = Person {
        name: "Peter",
        age: 27
    };
    println!("{:#?}", peter);
    println!("{}", Structure(3));
}