use message::Message;

fn main() {
    let q = Message::Quit;
    q.debug();

    let m = Message::Move { x: -5, y: 87 };
    m.debug();
    
    let w = Message::Write(String::from("hello"));
    w.debug();
    
    let cc = Message::ChangeColor(0, 1, 2);
    cc.debug();
}
