// Fixed list where elements are same dataypes

use std::mem;

pub fn run() {
        // datatype, len of array
    let mut arr: [i32;5] = [0,1,2,3,4];

    arr[2] = 50;

    println!("{:?}",arr);

    println!("{}",arr[0]);

    // Arrays are stack allocated
    println!("Array occupies {} bytes",mem::size_of_val(&arr));

    // get slices
    let slice: &[i32] = &arr[0..3];
    println!("Slice: {:?}",slice);
}