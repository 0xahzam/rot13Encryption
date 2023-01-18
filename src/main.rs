use std::io;

fn rot13(x:String){
    let mut fin = String::new();
    for i in x.chars() {
        let u = i as u8;
        if u > 96 && u < 110 {
            let u2 = u + 13;
            fin.push(u2 as char);
        } else if u > 109 && u < 123 {
            let u2 = u - 13;
            fin.push(u2 as char);
        } else if u > 64 && u < 78 {
            let u2 = u + 13;
            fin.push(u2 as char);
        } else if u > 77 && u < 91 {
            let u2 = u - 13;
            fin.push(u2 as char);
        } else {
            let u2 :u8= 32;
            fin.push(u2 as char);
        }
    }

    println!("{}",fin)
}

fn main() {
    println!("Type something...");
    let mut io = String::new();
    io::stdin().read_line(&mut io).expect("failed to readline");
    println!("");
    println!("ROT13 Encrypted...");
    rot13(io);
    println!{""}
    println!("Press enter to exit");
    io::stdin().read_line(&mut String::new()).unwrap();
}
