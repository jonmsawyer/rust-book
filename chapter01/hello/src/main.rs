use std::collections::HashMap;

fn main() {
    // string stuff
    
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    
    let s3 = s1 + &s2;
    
    println!("{}", s3);
    
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    
    // using a hashmap to count the number of words in a sentence
    
    let text = "hello world wonderful world";
    
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("{:?}", map);
}
