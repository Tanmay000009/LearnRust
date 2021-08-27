// pub means public
// fn for function

pub  fn run() {
    // Print to console
    println!("Hello from print.rs");
    println!("Number : {}",1);
    println!("Hi, myself {} a {}","Tanmay","Web D");
    println!("Hi, myself {0}. My name is {0}","Tanmay");

    // Named arguments
    println!("{name} is learning {code} ",name = "Tanmay",code = "RUST");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o} ",10,10,10);

    // Placeholder for debug traits
    println!("{:?}",(12,true,"Str"));

    // Basic math 
    println!("10 + 10 =  {}",10+10);
}