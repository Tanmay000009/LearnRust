// Tuples group together values of diffrent type
// Max 12 elements

pub fn run() {
    let person: (&str, &str,i8) = ("Tanmay","Surat",9);
    println!("{} is from {}, aged {}",person.0,person.1,person.2);
}