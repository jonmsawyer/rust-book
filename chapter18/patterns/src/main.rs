#![allow(dead_code)]

struct Point {
    x: i32,
    y: i32,
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message1 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Message3 {
    Hello { id: i32 },
}


// An example of pattern matching in a function definition. &(x, y) is the
// pattern.
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

// Using _ in the function signature
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    // A combination of `if let`, `else if`, `else if let`, and `else` pattern
    // matching.
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
    
    // An example of `while let` pattern matching.
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    
    // An example of `for` loop pattern matching. (index, value) is the
    // pattern.
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    
    // An example of `let` pattern matching. (x, y, z) is the pattern.
    let (x, y, z) = (1, 2, 3);
    println!("x = {}, y = {}, z = {}", x, y, z);
    
    // An example of pattern matching in functions. See the function
    // definition for more info.
    let point = (3, 5);
    print_coordinates(&point);
    
    // An example of a refutable pattern matching where irrefutable pattern
    // is necessary. Uncomment the following lines to see a compiler
    // error message.
    let some_option_value: Option<i32> = None;
    //let Some(x) = some_option_value;
    
    // This works though.
    if let Some(x) = some_option_value {
        println!("{}", x);
    }
    
    // The following `if let` pattern matching code will produce a warning.
    //if let x = 5 {
    //    println!("{}", x);
    //};
    
    // Pattern matching literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    
    // Matching named variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
    
    // Multiple patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    
    // Pattern matching using ..=
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    
    // Loop through inclusive ranges
    for x in 1..=5 {
        println!("x = {}", x);
    }
    
    // Cannot do ..=5
    //for x in ..=5 {
    //    println!("x = {}", x);
    //}
    
    // Can do ..=2 in slices
    let v = vec![1,2,3,4,5,6];
    
    println!("slice of v is {:?}", &v[..=2]);
    
    // Inclusive matching with char values with ..=
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
    
    // Destructuring a struct into variables `a` and `b`
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    
    // Destructuring a struct using field names `x` and `y`
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
    
    // Destructuring and matching literals in one pattern
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
    
    // Destructuring enum variants that hold different kinds of values
    let msg = Message1::ChangeColor(0, 160, 255);

    match msg {
        Message1::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message1::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message1::Write(text) => println!("Text message: {}", text),
        Message1::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }
    
    // Matching on nested enums
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message2::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
    
    // Destructuring a tuple and a struct
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    
    println!("feet = {}, inches = {}, Point {{ x = {}, y = {} }}", feet, inches, x, y);
    
    // Using _ in the function signature
    foo(3, 4);
    
    // Using an underscore within patterns that match Some variants
    // when we don’t need to use the value inside the Some.
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
    
    // Ignoring multiple parts of a tuple.
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
    
    // Starting a variable name with an underscore to avoid getting
    // unused variable warnings.
    let _x = 5;
    let _y = 10;
    
    // Using an underscore does not bind the value. As opposed to:
    // An unused variable starting with an underscore still binds
    // the value, which might take ownership of the value
    // ( if let Some(_s) = s { ).
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
    
    // Ignoring all fields of a Point except for x by using ..
    let origin = Point3D { x: 0, y: 0, z: 0 };

    match origin {
        Point3D { x, .. } => println!("x is {}", x),
    }
    
    // Matching only the first and last values in a tuple and
    // ignoring all other values.
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
    
    // Adding a match guard to a pattern. (if x < 5)
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
    
    // Using a match guard to test for equality with an outer
    // variable.
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
    
    // Combining multiple patterns with a match guard. (4 | 5 | 6 if y)
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
    
    // Using @ to bind to a value in a pattern while also testing it.
    // (id: id_variable @ 3..=7)
    let msg = Message3::Hello { id: 5 };

    match msg {
        Message3::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message3::Hello { id } => println!("Found some other id: {}", id),
    }
    
    //
    
}
