fn main() {
    // Constants
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is : {x}");
    }
    println!("The value of x is {x}");

    // Scalar Types(integers, floating-point numbers, Booleans, characters)
    // Integer Types: i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
    // Floating-point Types
    let x1 = 2.0; // f64
    let y1: f32 = 3.0; // f32

    // Compound Types(can group multiple values into one type)
    // The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x2, y2, z2) = tup;
    println!("The value of y2 is {y2}");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("x2: {five_hundred}, y2: {six_point_four}, z2: {one}");

    // The Array Type
    let arr = [1, 2, 3, 4, 5]; // arrays in Rust have a fixed length
    let arr2: [i32; 5] = [2, 3, 4, 5, 6];
    let arr3 = [3; 5]; // [3, 3, 3, 3, 3]
    let first = arr[0];
    let second = arr[1];

    // Functions
    another_function(5);

    let y3 = {
        let x3 = 3;
        x3 + 1
    }; // A new scope block is an expression

    println!("The value of y3 is: {y3}");

    let x3 = five();
    println!("The value of five function is: {x3}");

    // Control Flow
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number2 = if condition { 5 } else { 6 };
    println!("The value of number2 is: {number2}");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}"); // 20

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}"); // 2

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}
