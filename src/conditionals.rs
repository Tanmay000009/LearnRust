pub fn run() {
    let age = 18;

    if age>=18 {
        println!("Vote!!");
    } else {
        println!("Ineligible");
    }

    // Shorthand if
    let is_of_age = if age>=18 {true} else {false};
    println!("Is_of age: {}",is_of_age);
}