#![allow(dead_code)]

fn add_one(x: i32) -> i32 {
    x + 1
}

// Using the fn type to accept a function pointer as an argument.
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

// Returns closure, note the use of Box here.
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    // Using the fn type to accept a function pointer as an argument.
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
    
    // Here we create Status::Value instances using each u32 value
    // in the range that map is called on by using the initializer
    // function of Status::Value.
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    
    println!("list_of_statuses = {:?}", list_of_statuses);
}
