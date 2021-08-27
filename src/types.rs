pub fn run() {

    // by default compiler will treat it as i32
    let _x = 1;

    //by default "f64"
    let _y = 1.5;

    // add explicit type
    let _z: i64 = 1254;

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // boolean
    let is_active: bool =  true;

    let is_bigger = 10>3;

    //char
    let a1 = 'a';

    println!("{:?}",(_x,_y,_z,is_active,is_bigger,a1));
}