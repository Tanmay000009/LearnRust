// resizable arraay

use std::mem;

pub fn run() {
    let mut vec: Vec<i32> = vec![0,1,2,3,4];

    vec[2] = 50;

    // add to vector
    vec.push(5);
    vec.push(6);

    // pop
    vec.pop();

    println!("{:?}",vec);

    println!("{}",vec[0]);

    // Vector are stack allocated
    println!("Vector occupies {} bytes",mem::size_of_val(&vec));

    // get slices
    let slice: &[i32] = &vec[0..3];
    println!("Slice: {:?}",slice);

    // Loop throygh vector
    for x in vec.iter() {
        println!("Number: {}",x);
    }

    // Loop and mutate values
    for x in vec.iter_mut() {
        *x *= 2;
    }

    println!("{:?}",vec);
}