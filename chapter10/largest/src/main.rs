#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    
    fn y(&self) -> &U {
        &self.y
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

#[allow(dead_code)]
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[allow(dead_code)]
impl Point<f64, f64> {
    fn foo(&self) -> f64 {
        self.x
    }
}

#[allow(dead_code)]
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[allow(dead_code)]
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q', 'A', 'Z'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
    
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.012_f64, y: 4.044_f64 };
    println!("integer = {:?}", integer);
    println!("float = {:?}", float);
    
    let mixed = Point { x: 4, y: 3.50_f64 };
    println!("mixed = {:?}", mixed);
    
    println!("mixed.x() = {:?}, mixed.y() = {:?}", mixed.x(), mixed.y());
    println!("mixed.x = {:?}, mixed.y = {:?}", mixed.x, mixed.y);
    
    let point_f32 = Point { x: 0.0, y: 12.32 };
    println!("point_f32.foo() = {}", point_f32.foo());
    // Doing point_f32.foo() AND THEN doing point_f32.distance_from_origin() will result
    // in a compile time error because Rust infers the type based on the first usage of
    // a type's method signatures, even though there are reasonable defaults to a type's
    // primitive type fields.
    //println!("point_f32.distance_from_origin() = {}", point_f32.distance_from_origin());
    
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let foo = Point { x: 0.0, y: 1.01 };
    println!("foo = {:?}", foo);
}
