pub fn run() {
    greetings("hello", "Stalker");

    let s = add(1,2);
    println!("Sum: {}",s);

    // Closure
    // Functions will be block scoped. With closure u can have values out side the scope of function aswell
    let extra: i32 = 15;
    let add_nums = |n1: i32,n2: i32| n1 + n2+ extra;
    println!("Closure sum: {}",add_nums(1,5));
}

fn greetings(greet: &str,name: &str) {
    println!("{} {}",greet,name);
}

// -> tells datatype going to return
fn add(n1: i32,n2: i32) -> i32 {    
    // no ; means we gonna return that
    n1 + n2
}