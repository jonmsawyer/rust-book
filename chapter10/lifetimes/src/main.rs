use std::collections::HashMap;
use std::fmt::Display;


#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn most_of(sentence: &String, map: &mut HashMap<char, u32>) {
    for character in sentence.chars() {
        let count = map.entry(character).or_insert(0);
        *count += 1;
    }
}

fn main() {
    //{
    //    let r;                // ---------+-- 'a
    //                          //          |
    //    {                     //          |
    //        let x = 5;        // -+-- 'b  |
    //        r = &x;           //  |       |
    //    }                     // -+       |
    //                          //          |
    //    println!("r: {}", r); //          |
    //}                         // ---------+
    
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // This block of code won't compile
    //
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    let string = String::from("This is a super long line of text and the function should give me back a map of char to quantity values");
    let mut map = HashMap::<char, u32>::new();
    most_of(&string, &mut map);
    println!("map = {:?}", map);
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Important Excerpt: {:?}", i);
    println!("i.level() = {}", i.level());
    i.announce_and_return_part("foo bar beef");

    let longest_str = longest_with_an_announcement(
        "Is this the longest?",
        "Or is this the longest?",
        String::from("We're gonna see about the longest...")
    );
    println!("longest = {}", longest_str);
}
