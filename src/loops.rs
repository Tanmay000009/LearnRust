pub fn run() {
    let mut count =0;

    // Infinite loop
    loop {
        count += 1;
        println!("Number: {}",count);

        if count<=5 {break};
    }

    // While loop
    while count <= 10 {
        if count%2==0{
            println!("Number: {} is even",count);
        }
        count +=1;
    }

    // For loop
    for count in 0..10 {
        if count%2==0 {
            println!("Even: {}",count);
        }
    }
    println!("{}",count);
}