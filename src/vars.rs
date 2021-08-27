// Variables hold primitive data or refrences to data
// Variables are immutable by default
// Rust is a block scoped lang

pub fn run() {
    let name = "Tanmay";
    let mut age = 19;
    println!("My name is {}, and I am {}",name,age);

    age = 20;
    println!("My name is {}, and I am {}",name,age);

    // Define constants
    // Constants are all upper case, and also mention data types
    const ID: i32 = 1;
    println!("ID : {}",ID);

    // Asssign multiple vars
    let (my_name,my_age) = ("Tanmay",19);
    println!("My name is {}, I am {}",my_name,my_age);
}