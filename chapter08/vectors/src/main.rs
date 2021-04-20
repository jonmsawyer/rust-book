#[derive(Debug, Clone, Copy)]
struct Foo {
    x: i32,
    y: i32
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    let v: Vec<i32> = Vec::new();
    println!("v = {:?}", v);
    
    let v = vec![1, 2, 3];
    println!("v = {:?}", v);
    
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("v = {:?}", v);
    // do stuff with v
    
    let v = vec![1, 2, 3, 4, 5];

    //let third: &i32 = &v[2]; // works
    //let third = &v[2]; // works
    let third = v[2]; // works
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    
    let v = vec![1, 2, 3, 4, 5];
    
    // The next two lines makes this code panic!
    //let does_not_exist = &v[100];
    //let does_not_exist = v.get(100);
    
    let index = 433;
    match v.get(index) {
        Some(value) => println!("v[{}] = {}", index, value),
        None => println!("v[{}] = None", index),
    }
    
    let mut v = vec![1, 2, 3, 4, 5];

    //let first = &v[0]; // doesn't work
    let first = v[0]; // what makes this different? It has to do with reference vs copy
                      // and Clone and Copy traits are already defined for i32 types.

    println!("v[] = {:?}", v);
    v.push(6);
    println!("v[] = {:?}", v);
    v.clear();
    println!("v[] = {:?}", v);
    
    println!("The first element is: {}", first);
    
    let mut foo = Vec::<Foo>::new();
    
    foo.push(Foo{ x: 100, y: 5000 });
    foo.push(Foo{ x: 3, y: 4});
    println!("foo = {:?}", foo);
    
    let v = vec![100, 32, 57];
    //for i in v { // Note Clone/Copy here
    for i in &v { // Note reference here
        println!("{}", i);
    }
    
    let mut v = vec![
        Foo { x: 1, y: 2 },
        Foo { x: 3, y: 4 },
        Foo { x: 5, y: 6 },
        Foo { x: 7, y: 8 },
    ];

    //let first = &v[0]; // doesn't work
    let first = v[0]; // Copy and Clone Trait is derived, so can copy

    println!("v[] = {:?}", v);
    v.push(Foo { x: 1000, y: 300000 });
    println!("v[] = {:?}", v);
    v.clear();
    println!("v[] = {:?}", v);
    
    println!("The first element is: {:?}", first);
    
    // iterating over mutable references
    let mut v = vec![100, 32, 57];
    println!("v = {:?}", v);
    for i in &mut v {
        *i += 50;
    }
    println!("v = {:?}", v);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
    println!("row = {:?}", row);
} // <- v goes out of scope and is freed here
