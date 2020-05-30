use std::slice;

// Defining and using an immutable static variable.
static HELLO_WORLD: &str = "Hello, world!";

// Reading from or writing to a mutable static variable is unsafe.
static mut COUNTER: u32 = 0;

// Defining an unsafe trait.
unsafe trait Foo {
    // methods go here
}

// Implementing an unsafe trait.
unsafe impl Foo for i32 {
    // method implementations go here
}

// Reading from or writing to a mutable static variable is unsafe.
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe fn dangerous() {
    println!("in dangerous()");
}

// Using unsafe code in the implementation of the split_at_mut function.
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Declaring and calling an extern function defined in another language.
extern "C" {
    fn abs(input: i32) -> i32;
}

// We make the call_from_c function accessible from C code, after
// it's compiled to a shared library and linked from C.
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

fn main() {
    // Dereferencing raw pointers within an unsafe block.
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    
    // Calling an unsafe function within an unsafe block
    unsafe {
        dangerous();
    }
    
    // Using the safe split_at_mut function on a vec![].
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    
    // Using unsafe code in the implementation of the split_at_mut function.
    let (a, b) = split_at_mut(r, 3);
    
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    
    // Creating a slice from an arbitrary memory location.
    // Warning: undefined behavior
    let address = 0x01234usize;
    let r = address as *mut i32;

    let _slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    
    // Declaring and calling an extern function defined in another language.
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
    
    // Defining and using an immutable static variable.
    println!("name is: {}", HELLO_WORLD);
    
    // Reading from or writing to a mutable static variable is unsafe.
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
