#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrString {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrSplit {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    
    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);
    
    let home2 = IpAddrString::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddrString::V6(String::from("::1"));
    
    println!("home2: {:?}", home2);
    println!("loopback2: {:?}", loopback2);
    
    let home3 = IpAddrSplit::V4(127, 0, 0, 1);
    let loopback3 = IpAddrSplit::V6(String::from("::1"));
    
    println!("home3: {:?}", home3);
    println!("loopback3: {:?}", loopback3);
}
