fn main() {
    let mut s = String::new();
    s.push_str("foo bar");
    println!("{}", s);
    
    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);
    
    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    println!("{}", s);
    
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");
    
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    
    let mut s = String::from("lo");
    s.push('l');
    println!("s = {}", s);
    
    let s1 = String::from("Hello, ");
    let mut s2 = String::from("world!");
    //let s3 = &s1 + &s2; // won't work
    let s1 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s1 = {}", s1);
    s2.push_str(" har har");
    println!("s2 = {}", s2);
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = s1 + "-" + &s2 + "-" + &s3;
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = format!("{}-{}-{}", s1, s2, s3);
    
    let _s1 = String::from("hello");
    //let h = s1[0]; // won't work
    
    let _hello = String::from("Hola");
    let _hello = String::from("Здравствуйте");
    
    let hello = "Здравствуйте";
    // hello = &hello[0]; // won't work, `З` is 2 bytes
    let answer = &hello[0..2];
    
    println!("{}", answer);
    
    // Bytes (.bytes()):
    // [224, 164, 168, 224, 164, 174, 224, 164, 184,
    //  224, 165, 141, 224, 164, 164, 224, 165, 135]
    //
    // Unicode scalars (.chars()):
    // ['न', 'म', 'स', '्', 'त', 'े']
    //
    // "Letters":
    // ["न", "म", "स्", "ते"]
    // Getting grapheme clusters from strings is complex, so this
    // functionality is not provided by the standard library. Crates
    // are available on crates.io if this is the functionality you
    // need.
    let devanagari = "नमस्ते"; // Hindi word written in Devanagari
    
    for c in devanagari.chars() {
        println!("{}", c);
    }
    
    for b in devanagari.bytes() {
        println!("{}", b);
    }
}
