use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self { 
            name: name.to_lowercase(), 
            greeting: greeting.to_string()
        }
    }
    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name
        .trim()
        .to_lowercase()
}

fn main() {
    // let mut allow_them_in = false;
    
    println!("Hello, what's your name?");
    let name = what_is_your_name();
    
    let visitor_list = vec![
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hi Steve. Your milk is in the fridge."),
        Visitor::new("fred", "Wow, who invited Fred?"),
    ];

    let known_visitor = visitor_list
        .iter()
        .find(|visitor| visitor.name == name);
    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("You are not on the visitor list. Please leave.")
    }
    // for visitor in &visitor_list {
    //     if visitor == &name {
    //         allow_them_in = true;
    //     }
    // }

    // if allow_them_in == true {
    //     println!("Hello, {}", name);
    // } else {
    //     println!("Sorry, you are not on the list.");
    // }
}
