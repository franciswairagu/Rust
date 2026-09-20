use std::io;

fn main() {
    // DATA TYPES
    // Scalar types
    // Integers
    let a: u32 = 5;
    let b: i32 = -5; //Default to i32
    
    // Floating point
    let c = 2.0; // Defaults to f64
    let d: f32 = 3.0;

    println!("a: {a}");
    println!("b: {b}");
    println!("c: {c}");
    println!("d: {d}");

    // Numerric operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    println!("Sum: {sum}");
    println!("Difference: {difference}");
    println!("Product: {product}");
    println!("Quotient: {quotient}");
    println!("Truncated: {truncated}");
    println!("Remainder: {remainder}");

    // Boolean
    // let t = true;
    // let f: bool = false;

    // Character type
    let z: char = 'Z'; // Uses single quotes as opposed to str
    println!("Char: {z}");

    // COMPOUND TYPES
    // Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple: {tup:?}");
    println!("First: {}, Second: {}, Third: {}", tup.0, tup.1, tup.2);    
    
    let tup2 = (100,200,300);
    let (i,j,k) = tup2;
    println!("{i}, {j}, {k}");

    // Array Type
    let numbers = [1,2,3,4,5];
    println!("{numbers:?}");

    let threes = [3; 5];
    println!("{threes:?}");
    // indexing
    let first = threes[0];
    println!("{first}");

    let array = [1, 2, 3, 4, 5];
    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = array[index];
    println!("The value of the element at inde {index} is {element}");
}
