// Structs used to create custom data types

// Tradtional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8  
}

// Tuple struct
struct Colors(u8,u8,u8);

// struct with function
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct a person
    fn construct_person(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }
    
    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}",self.first_name,self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self,last: &str) {
        self.last_name = last.to_string();
    }

    // name to tuple
    fn to_tuple(self) -> (String,String) {
        (self.first_name,self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };
    c.red = 200;
    println!("Color: {} {} {}",c.red,c.green,c.blue);

    let mut col = Colors(255,0,0);
    col.0 = 100;
    println!("Colors: {} {} {}",col.0,col.1,col.2);

    let mut person = Person::construct_person("Unknown","Anonymous");
    println!("Person {} {} created",person.first_name,person.last_name);
    println!("Full name: {}",person.full_name());
    person.set_last_name("Nobody knows");
    println!("Full name: {}",person.full_name());
    println!("Name tuple: {:?}",person.to_tuple());
}