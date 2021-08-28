// Primitve str = Imutable fixed-length string somewhere in memory

// String = Growable, heap allocated data structure : Use when you need to modify or own string data

pub fn run() {
    let mut hello = String::from("hello");
    println!("{}",hello);

    // get length
    println!("Lenght: {}",hello.len());

    // .push() for unicode chars
    hello.push('a');

    // push string
    hello.push_str("!!!");

    println!("{}",hello);

    // Contains
    println!("Conains ll: {}",hello.contains("ll"));

    // Replace
    println!("Replaced: {}",hello.replace("a!!!",""));


    //  strip by whitespace
    hello.push_str(" World! Hola");
    for word in hello.split_whitespace() {
        println!("{}",word);
    }

    // string with capacity
    let mut str = String::with_capacity(10);
    str.push('a');
    str.push('b');

    println!("{}",str);

    // Assertion testing
    // if l!=r , program will give error or else mast chalega
    assert_eq!(str,"ab");

    //assert_eq!("Aman","Tanmay");
}